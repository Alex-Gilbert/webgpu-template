use image::{GenericImageView, ImageResult};
use serde::Deserialize;

// Default sampler configuration when no TOML is provided
const DEFAULT_SAMPLER_DESCRIPTOR: wgpu::SamplerDescriptor = wgpu::SamplerDescriptor {
    label: Some("default_sampler"),
    address_mode_u: wgpu::AddressMode::ClampToEdge,
    address_mode_v: wgpu::AddressMode::ClampToEdge,
    address_mode_w: wgpu::AddressMode::ClampToEdge,
    mag_filter: wgpu::FilterMode::Linear,
    min_filter: wgpu::FilterMode::Nearest,
    mipmap_filter: wgpu::FilterMode::Nearest,
    lod_min_clamp: 0.0,
    lod_max_clamp: 0.0,
    compare: None,
    anisotropy_clamp: 1,
    border_color: None,
};

// Metadata structures (unchanged)
#[derive(Debug, Deserialize, Default)]
pub struct TextureMetadata {
    pub label: Option<String>,
    pub format: Option<String>,
    pub dimension: Option<String>,
    pub generate_mipmaps: Option<bool>,
    pub sampler: Option<SamplerConfig>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct SamplerConfig {
    // Address modes
    pub address_mode_u: Option<String>,
    pub address_mode_v: Option<String>,
    pub address_mode_w: Option<String>,

    // Filtering modes
    pub mag_filter: Option<String>,
    pub min_filter: Option<String>,
    pub mipmap_filter: Option<String>,

    // LOD clamps
    pub lod_min_clamp: Option<f32>,
    pub lod_max_clamp: Option<f32>,

    // Other sampler properties
    pub compare: Option<String>,
    pub anisotropy_clamp: Option<u16>,
    pub border_color: Option<String>,
}

// Define a texture usage enum to make the intent clearer
pub enum TextureUsageType {
    Standard,
    ComputeOutput,
    DepthTexture,
    RenderTarget(u32), // Sample count
}

#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub dimensions: (u32, u32),
}

// Builder for texture creation
pub struct TextureBuilder<'a> {
    device: &'a wgpu::Device,
    queue: Option<&'a wgpu::Queue>,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    label: String,
    dimension: wgpu::TextureDimension,
    mip_level_count: u32,
    usage_type: TextureUsageType,
    sampler_config: Option<SamplerConfig>,
    data: Option<&'a [u8]>,
}

impl<'a> TextureBuilder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        Self {
            device,
            queue: None,
            width: 1,
            height: 1,
            format: wgpu::TextureFormat::Rgba8Unorm,
            label: "texture".to_string(),
            dimension: wgpu::TextureDimension::D2,
            mip_level_count: 1,
            usage_type: TextureUsageType::Standard,
            sampler_config: None,
            data: None,
        }
    }

    // Setter methods for all properties
    pub fn queue(mut self, queue: &'a wgpu::Queue) -> Self {
        self.queue = Some(queue);
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn format(mut self, format: wgpu::TextureFormat) -> Self {
        self.format = format;
        self
    }

    pub fn format_str(mut self, format: Option<&str>) -> Self {
        if let Some(format_str) = format {
            self.format = parse_texture_format(&Some(format_str.to_string()));
        }
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn dimension(mut self, dimension: wgpu::TextureDimension) -> Self {
        self.dimension = dimension;
        self
    }

    pub fn dimension_str(mut self, dimension: Option<&str>) -> Self {
        if let Some(dim_str) = dimension {
            self.dimension = parse_texture_dimension(&Some(dim_str.to_string()));
        }
        self
    }

    pub fn mip_level_count(mut self, count: u32) -> Self {
        self.mip_level_count = count;
        self
    }

    pub fn auto_mipmaps(mut self, enable: bool) -> Self {
        if enable {
            let max_dimension = self.width.max(self.height);
            self.mip_level_count = (max_dimension as f32).log2().floor() as u32 + 1;
        } else {
            self.mip_level_count = 1;
        }
        self
    }

    pub fn usage_type(mut self, usage_type: TextureUsageType) -> Self {
        self.usage_type = usage_type;
        self
    }

    pub fn sampler_config(mut self, config: SamplerConfig) -> Self {
        self.sampler_config = Some(config);
        self
    }

    pub fn data(mut self, data: &'a [u8]) -> Self {
        self.data = Some(data);
        self
    }

    // Factory methods for common texture types
    pub fn compute_output(mut self) -> Self {
        self.usage_type = TextureUsageType::ComputeOutput;
        self
    }

    pub fn depth_texture(mut self) -> Self {
        self.usage_type = TextureUsageType::DepthTexture;
        self.format = wgpu::TextureFormat::Depth32Float;
        self
    }

    pub fn render_target(mut self, sample_count: u32) -> Self {
        self.usage_type = TextureUsageType::RenderTarget(sample_count);
        self
    }

    // Build the texture
    pub fn build(self) -> Result<Texture, String> {
        let size = wgpu::Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: 1,
        };

        // Determine usage and sample count based on usage type
        let (usage, sample_count) = match self.usage_type {
            TextureUsageType::Standard => {
                let mut usage = wgpu::TextureUsages::TEXTURE_BINDING;
                if self.data.is_some() {
                    usage |= wgpu::TextureUsages::COPY_DST;
                }
                if self.mip_level_count > 1 {
                    usage |= wgpu::TextureUsages::RENDER_ATTACHMENT;
                }
                (usage, 1)
            }
            TextureUsageType::ComputeOutput => (
                wgpu::TextureUsages::STORAGE_BINDING
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::COPY_DST,
                1,
            ),
            TextureUsageType::DepthTexture => (
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                1,
            ),
            TextureUsageType::RenderTarget(count) => (
                wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
                count,
            ),
        };

        // Create the texture
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&self.label),
            size,
            mip_level_count: self.mip_level_count,
            sample_count,
            dimension: self.dimension,
            format: self.format,
            usage,
            view_formats: &[],
        });

        // Write data if provided and we have a queue
        if let (Some(data), Some(queue)) = (self.data, self.queue) {
            if let Ok(img) = image::load_from_memory(data) {
                let rgba = img.to_rgba8();

                queue.write_texture(
                    wgpu::ImageCopyTexture {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &rgba,
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * self.width),
                        rows_per_image: Some(self.height),
                    },
                    size,
                );

                // Generate mipmaps if needed
                if self.mip_level_count > 1 {
                    generate_mipmaps(
                        self.device,
                        queue,
                        &texture,
                        self.format,
                        self.mip_level_count,
                        size,
                    );
                }
            } else {
                return Err("Failed to load image data".to_string());
            }
        }

        // Create view
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("{}_view", self.label)),
            format: None,
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: Some(self.mip_level_count),
            base_array_layer: 0,
            array_layer_count: None,
        });

        // Create sampler
        let sampler_descriptor_label = format!("{}_sampler", self.label);
        let sampler = self.device.create_sampler(&create_sampler_descriptor(
            Some(&sampler_descriptor_label),
            &self.sampler_config,
        ));

        Ok(Texture {
            texture,
            view,
            sampler,
            dimensions: (self.width, self.height),
        })
    }
}

// Simplified implementation of Texture using the builder
impl Texture {
    pub fn new_from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        metadata: Option<TextureMetadata>,
    ) -> ImageResult<Self> {
        let img = image::load_from_memory(bytes)?;
        let dimensions = img.dimensions();
        let metadata = metadata.unwrap_or_default();

        let mut builder = TextureBuilder::new(device)
            .queue(queue)
            .size(dimensions.0, dimensions.1)
            .data(bytes);

        // Apply metadata if provided
        if let Some(label) = &metadata.label {
            builder = builder.label(label);
        } else {
            builder = builder.label("memory_texture");
        }

        if let Some(format) = &metadata.format {
            builder = builder.format(parse_texture_format(&Some(format.clone())));
        }

        if let Some(dimension) = &metadata.dimension {
            builder = builder.dimension(parse_texture_dimension(&Some(dimension.clone())));
        }

        if let Some(generate_mipmaps) = metadata.generate_mipmaps {
            builder = builder.auto_mipmaps(generate_mipmaps);
        }

        if let Some(sampler) = metadata.sampler {
            builder = builder.sampler_config(sampler);
        }

        builder.build().map_err(|e| {
            image::ImageError::Unsupported(image::error::UnsupportedError::from_format_and_kind(
                image::error::ImageFormatHint::Name("wgpu texture creation".to_string()),
                image::error::UnsupportedErrorKind::GenericFeature(e),
            ))
        })
    }

    pub fn new_compute_output(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: Option<wgpu::TextureFormat>,
        label: Option<&str>,
        sampler_config: Option<SamplerConfig>,
    ) -> Self {
        let mut builder = TextureBuilder::new(device)
            .size(width, height)
            .compute_output()
            .label(label.unwrap_or("compute_output_texture"));

        if let Some(fmt) = format {
            builder = builder.format(fmt);
        }

        if let Some(config) = sampler_config {
            builder = builder.sampler_config(config);
        }

        builder
            .build()
            .expect("Failed to create compute output texture")
    }

    pub fn new_depth_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        label: Option<&str>,
        sampler_config: Option<SamplerConfig>,
    ) -> Self {
        let mut builder = TextureBuilder::new(device)
            .size(width, height)
            .depth_texture()
            .label(label.unwrap_or("depth_texture"));

        if let Some(config) = sampler_config {
            builder = builder.sampler_config(config);
        }

        builder.build().expect("Failed to create depth texture")
    }

    pub fn new_render_target(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: Option<wgpu::TextureFormat>,
        label: Option<&str>,
        sampler_config: Option<SamplerConfig>,
        sample_count: u32,
    ) -> Self {
        let mut builder = TextureBuilder::new(device)
            .size(width, height)
            .render_target(sample_count)
            .label(label.unwrap_or("render_target"));

        if let Some(fmt) = format {
            builder = builder.format(fmt);
        }

        if let Some(config) = sampler_config {
            builder = builder.sampler_config(config);
        }

        builder
            .build()
            .expect("Failed to create render target texture")
    }
}

// Helper functions (with minimal changes from original)
fn generate_mipmaps(
    _device: &wgpu::Device,
    _queue: &wgpu::Queue,
    _texture: &wgpu::Texture,
    _format: wgpu::TextureFormat,
    _mip_level_count: u32,
    _size: wgpu::Extent3d,
) {
    // Same placeholder implementation as before
    println!("Note: Mipmap generation requested but not fully implemented.");
}

// Helper functions for parsing values from strings (unchanged)
fn parse_texture_format(format_str: &Option<String>) -> wgpu::TextureFormat {
    match format_str {
        Some(format) => match format.as_str() {
            "Rgba8Unorm" => wgpu::TextureFormat::Rgba8Unorm,
            "Rgba8UnormSrgb" => wgpu::TextureFormat::Rgba8UnormSrgb,
            "Bgra8Unorm" => wgpu::TextureFormat::Bgra8Unorm,
            "Bgra8UnormSrgb" => wgpu::TextureFormat::Bgra8UnormSrgb,
            "Rgb10a2Unorm" => wgpu::TextureFormat::Rgb10a2Unorm,
            _ => {
                eprintln!(
                    "Warning: Unknown texture format '{}', using Rgba8Unorm",
                    format
                );
                wgpu::TextureFormat::Rgba8Unorm
            }
        },
        None => wgpu::TextureFormat::Rgba8Unorm,
    }
}

fn parse_texture_dimension(dimension_str: &Option<String>) -> wgpu::TextureDimension {
    match dimension_str {
        Some(dim) => match dim.as_str() {
            "D1" => wgpu::TextureDimension::D1,
            "D2" => wgpu::TextureDimension::D2,
            "D3" => wgpu::TextureDimension::D3,
            _ => {
                eprintln!("Warning: Unknown texture dimension '{}', using D2", dim);
                wgpu::TextureDimension::D2
            }
        },
        None => wgpu::TextureDimension::D2,
    }
}

fn create_sampler_descriptor<'a>(
    label: Option<&'a str>,
    config: &Option<SamplerConfig>,
) -> wgpu::SamplerDescriptor<'a> {
    let mut descriptor = wgpu::SamplerDescriptor {
        label,
        ..DEFAULT_SAMPLER_DESCRIPTOR
    };

    if let Some(config) = config {
        // Parse address modes
        if let Some(mode) = &config.address_mode_u {
            descriptor.address_mode_u = parse_address_mode(mode);
        }
        if let Some(mode) = &config.address_mode_v {
            descriptor.address_mode_v = parse_address_mode(mode);
        }
        if let Some(mode) = &config.address_mode_w {
            descriptor.address_mode_w = parse_address_mode(mode);
        }

        // Parse filter modes
        if let Some(filter) = &config.mag_filter {
            descriptor.mag_filter = parse_filter_mode(filter);
        }
        if let Some(filter) = &config.min_filter {
            descriptor.min_filter = parse_filter_mode(filter);
        }
        if let Some(filter) = &config.mipmap_filter {
            descriptor.mipmap_filter = parse_filter_mode(filter);
        }

        // Set LOD clamps
        if let Some(clamp) = config.lod_min_clamp {
            descriptor.lod_min_clamp = clamp;
        }
        if let Some(clamp) = config.lod_max_clamp {
            descriptor.lod_max_clamp = clamp;
        }

        // Parse compare function
        if let Some(compare) = &config.compare {
            descriptor.compare = parse_compare_function(compare);
        }

        // Set anisotropy clamp
        if let Some(clamp) = config.anisotropy_clamp {
            descriptor.anisotropy_clamp = clamp;
        }

        // Parse border color
        if let Some(color) = &config.border_color {
            descriptor.border_color = parse_border_color(color);
        }
    }

    descriptor
}

// The rest of the helper functions (unchanged)
fn parse_address_mode(mode: &str) -> wgpu::AddressMode {
    match mode {
        "ClampToEdge" => wgpu::AddressMode::ClampToEdge,
        "Repeat" => wgpu::AddressMode::Repeat,
        "MirrorRepeat" => wgpu::AddressMode::MirrorRepeat,
        "ClampToBorder" => wgpu::AddressMode::ClampToBorder,
        _ => {
            eprintln!(
                "Warning: Unknown address mode '{}', using ClampToEdge",
                mode
            );
            wgpu::AddressMode::ClampToEdge
        }
    }
}

fn parse_filter_mode(mode: &str) -> wgpu::FilterMode {
    match mode {
        "Nearest" => wgpu::FilterMode::Nearest,
        "Linear" => wgpu::FilterMode::Linear,
        _ => {
            eprintln!("Warning: Unknown filter mode '{}', using Linear", mode);
            wgpu::FilterMode::Linear
        }
    }
}

fn parse_compare_function(func: &str) -> Option<wgpu::CompareFunction> {
    match func {
        "Never" => Some(wgpu::CompareFunction::Never),
        "Less" => Some(wgpu::CompareFunction::Less),
        "Equal" => Some(wgpu::CompareFunction::Equal),
        "LessEqual" => Some(wgpu::CompareFunction::LessEqual),
        "Greater" => Some(wgpu::CompareFunction::Greater),
        "NotEqual" => Some(wgpu::CompareFunction::NotEqual),
        "GreaterEqual" => Some(wgpu::CompareFunction::GreaterEqual),
        "Always" => Some(wgpu::CompareFunction::Always),
        "None" => None,
        _ => {
            eprintln!("Warning: Unknown compare function '{}', using None", func);
            None
        }
    }
}

fn parse_border_color(color: &str) -> Option<wgpu::SamplerBorderColor> {
    match color {
        "TransparentBlack" => Some(wgpu::SamplerBorderColor::TransparentBlack),
        "OpaqueBlack" => Some(wgpu::SamplerBorderColor::OpaqueBlack),
        "OpaqueWhite" => Some(wgpu::SamplerBorderColor::OpaqueWhite),
        "None" => None,
        _ => {
            eprintln!("Warning: Unknown border color '{}', using None", color);
            None
        }
    }
}

// Keep the include_texture macro (unchanged)
#[macro_export]
macro_rules! include_texture {
    ($path:expr, $device:expr, $queue:expr) => {{
        const METADATA_PATH: &str = concat!($path, ".toml");

        // Include bytes and metadata at compile time
        let bytes = include_bytes!($path);

        // Load the metadata if it exists
        #[allow(unused_variables)]
        let metadata_opt = {
            let toml_content = include_str!(concat!($path, ".toml"));
            match toml::from_str(toml_content) {
                Ok(meta) => Some(meta),
                Err(err) => {
                    eprintln!(
                        "Warning: Failed to parse metadata file {}: {}",
                        METADATA_PATH, err
                    );
                    None
                }
            }
        };

        // Use from_bytes with the compile-time included data
        match $crate::utils::texture::Texture::new_from_bytes($device, $queue, bytes, metadata_opt)
        {
            Ok(texture) => texture,
            Err(err) => panic!("Failed to load texture: {:?}", err),
        }
    }};
}

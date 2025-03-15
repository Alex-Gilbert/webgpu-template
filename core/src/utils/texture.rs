use image::{GenericImageView, ImageResult};
use serde::Deserialize;
// use std::fs;
// use std::path::{Path, PathBuf};

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

// Metadata structure to be parsed from TOML files
#[derive(Debug, Deserialize, Default)]
pub struct TextureMetadata {
    // Label for the texture (optional, defaults to filename)
    pub label: Option<String>,

    // Texture format (optional, defaults to Rgba8Unorm)
    pub format: Option<String>,

    // Texture dimension (optional, defaults to D2)
    pub dimension: Option<String>,

    // Generate mipmaps (optional, defaults to false)
    pub generate_mipmaps: Option<bool>,

    // Sampler configuration (optional, uses defaults if not specified)
    pub sampler: Option<SamplerConfig>,
}

#[derive(Debug, Deserialize, Default)]
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

#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub metadata: TextureMetadata,
    pub dimensions: (u32, u32),
}

impl Texture {
    /// Creates a texture from bytes with optional metadata
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        metadata: Option<TextureMetadata>,
    ) -> ImageResult<Self> {
        let img = image::load_from_memory(bytes)?;
        let dimensions = img.dimensions();

        let metadata = metadata.unwrap_or_default();

        // Determine the label
        let label = metadata
            .label
            .clone()
            .unwrap_or_else(|| "memory_texture".to_string());

        // Determine the format
        let format = parse_texture_format(&metadata.format);

        // Create sampler descriptor from metadata
        let descriptor_label = format!("{}_sampler", label);
        let descriptor_label = descriptor_label.as_str();
        let sampler_descriptor =
            create_sampler_descriptor(Some(descriptor_label), &metadata.sampler);

        // Determine mip level count
        let mip_level_count = if metadata.generate_mipmaps.unwrap_or(false) {
            let max_dimension = dimensions.0.max(dimensions.1);
            (max_dimension as f32).log2().floor() as u32 + 1
        } else {
            1
        };

        // Create the texture
        let texture = create_texture(
            device,
            queue,
            &img,
            format,
            &label,
            mip_level_count,
            parse_texture_dimension(&metadata.dimension),
        )?;

        Ok(Self {
            texture: texture.0,
            view: texture.1,
            sampler: device.create_sampler(&sampler_descriptor),
            metadata,
            dimensions,
        })
    }
}

// Helper function to create the actual texture
fn create_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    img: &image::DynamicImage,
    format: wgpu::TextureFormat,
    label: &str,
    mip_level_count: u32,
    dimension: wgpu::TextureDimension,
) -> ImageResult<(wgpu::Texture, wgpu::TextureView)> {
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count,
        sample_count: 1,
        dimension,
        format,
        usage: wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::TEXTURE_BINDING
            | if mip_level_count > 1 {
                wgpu::TextureUsages::RENDER_ATTACHMENT
            } else {
                wgpu::TextureUsages::empty()
            },
        view_formats: &[],
    });

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
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        size,
    );

    // Generate mipmaps if needed
    if mip_level_count > 1 {
        generate_mipmaps(device, queue, &texture, format, mip_level_count, size);
    }

    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some(&format!("{}_view", label)),
        format: None,
        dimension: None,
        aspect: wgpu::TextureAspect::All,
        base_mip_level: 0,
        mip_level_count: Some(mip_level_count),
        base_array_layer: 0,
        array_layer_count: None,
    });

    Ok((texture, view))
}

// Helper function to generate mipmaps
// Note: This is a simplified version - a real implementation would need a proper compute or render pass
fn generate_mipmaps(
    _device: &wgpu::Device,
    _queue: &wgpu::Queue,
    _texture: &wgpu::Texture,
    _format: wgpu::TextureFormat,
    _mip_level_count: u32,
    _size: wgpu::Extent3d,
) {
    // In a real implementation, you would:
    // 1. Create a compute shader or render pipeline to generate mipmaps
    // 2. Dispatch the shader to generate each mip level

    // This is just a placeholder - a real implementation would be more complex
    println!("Note: Mipmap generation requested but not fully implemented.");

    // TODO: Implement proper mipmap generation
    // For a proper implementation, see examples like wgpu-rs/examples/texture-arrays
}

// Helper function to parse texture format from string
fn parse_texture_format(format_str: &Option<String>) -> wgpu::TextureFormat {
    match format_str {
        Some(format) => match format.as_str() {
            "Rgba8Unorm" => wgpu::TextureFormat::Rgba8Unorm,
            "Rgba8UnormSrgb" => wgpu::TextureFormat::Rgba8UnormSrgb,
            "Bgra8Unorm" => wgpu::TextureFormat::Bgra8Unorm,
            "Bgra8UnormSrgb" => wgpu::TextureFormat::Bgra8UnormSrgb,
            "Rgb10a2Unorm" => wgpu::TextureFormat::Rgb10a2Unorm,
            // Add other formats as needed
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

// Helper function to parse texture dimension from string
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

// Helper function to create a sampler descriptor from metadata
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

// Helper function to parse address mode from string
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

// Helper function to parse filter mode from string
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

// Helper function to parse compare function from string
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

// Helper function to parse border color from string
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

// Load image macro with compile-time IO
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
        match $crate::utils::texture::Texture::from_bytes($device, $queue, bytes, metadata_opt) {
            Ok(texture) => texture,
            Err(err) => panic!("Failed to load texture: {:?}", err),
        }
    }};
}

// Example TOML metadata file (diffuse.toml)
/*
# Texture metadata
label = "Diffuse Map"
format = "Rgba8UnormSrgb"
generate_mipmaps = true

[sampler]
address_mode_u = "Repeat"
address_mode_v = "Repeat"
mag_filter = "Linear"
min_filter = "Linear"
mipmap_filter = "Linear"
anisotropy_clamp = 16
*/

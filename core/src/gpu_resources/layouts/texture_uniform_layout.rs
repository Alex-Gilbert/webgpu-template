use bevy_ecs::system::Resource;

use crate::utils::texture::Texture;

#[derive(Resource)]
pub struct TextureUniformLayout<const N: usize> {
    pub layout: wgpu::BindGroupLayout,
}

impl<const N: usize> TextureUniformLayout<N> {
    pub fn new(device: &wgpu::Device) -> Self {
        // Generate entries dynamically based on N (number of texture-sampler pairs)
        let mut entries = Vec::with_capacity(N * 2);

        for i in 0..N {
            // Add texture binding
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: (i * 2) as u32,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            });

            // Add sampler binding
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: (i * 2 + 1) as u32,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            });
        }

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&format!("texture_bind_group_layout_{}pairs", N)),
            entries: &entries,
        });

        Self {
            layout: bind_group_layout,
        }
    }

    /// Creates a bind group for a single texture-sampler pair at the specified index
    pub fn create_bind_group_for_slot(
        &self,
        device: &wgpu::Device,
        texture: &Texture,
        slot_index: usize,
    ) -> wgpu::BindGroup {
        assert!(slot_index < N, "Slot index out of bounds");

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("texture_bind_group_slot_{}", slot_index)),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: (slot_index * 2) as u32,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: (slot_index * 2 + 1) as u32,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        })
    }

    /// Creates a bind group with multiple textures, filling all slots
    pub fn create_complete_bind_group(
        &self,
        device: &wgpu::Device,
        textures: &[&Texture; N],
    ) -> wgpu::BindGroup {
        let mut entries = Vec::with_capacity(N * 2);

        for (i, texture) in textures.iter().enumerate() {
            entries.push(wgpu::BindGroupEntry {
                binding: (i * 2) as u32,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            });

            entries.push(wgpu::BindGroupEntry {
                binding: (i * 2 + 1) as u32,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            });
        }

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("complete_texture_bind_group"),
            layout: &self.layout,
            entries: &entries,
        })
    }
}

// Usage examples:

// For a simple diffuse-only material with one texture-sampler pair:
// type DiffuseBindGroupLayout = TextureBindGroupLayout<1>;

// For a PBR material with albedo, normal, metallic-roughness, and emission:
// type PbrBindGroupLayout = TextureBindGroupLayout<4>;

use bevy_ecs::{system::Resource, world::World};

use crate::gpu_resources::layouts::camera_uniform_layout::CameraUniformLayout;
use crate::gpu_resources::layouts::model_uniform_layout::ModelUniformLayout;
use crate::gpu_resources::layouts::texture_uniform_layout::TextureUniformLayout;
use crate::gpu_resources::render_resources::RenderResources;
use crate::gpu_resources::types::basic_vertex::BasicVertex;

use super::super::shaders::unlit_diffuse::SHADER_DESCRIPTOR_FRAGMENT;
use super::super::shaders::unlit_diffuse::SHADER_DESCRIPTOR_VERTEX;

#[derive(Resource)]
pub struct UnlitDiffusePipeline {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl UnlitDiffusePipeline {
    pub fn new(world: &World) -> Self {
        let render_resources = world.get_resource::<RenderResources>().unwrap();
        let device = &render_resources.device;

        let texture_uniform_layout = &world
            .get_resource::<TextureUniformLayout<1>>()
            .unwrap()
            .layout;
        let model_uniform_layout = &world.get_resource::<ModelUniformLayout>().unwrap().layout;
        let camera_uniform_layout = &world.get_resource::<CameraUniformLayout>().unwrap().layout;

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("unlit_diffuse_pipeline_layout"),
            bind_group_layouts: &[
                camera_uniform_layout,
                model_uniform_layout,
                texture_uniform_layout,
            ],
            push_constant_ranges: &[],
        });

        let vertex_shader_module = device.create_shader_module(SHADER_DESCRIPTOR_VERTEX);
        let fragment_shader_module = device.create_shader_module(SHADER_DESCRIPTOR_FRAGMENT);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("unlit_diffuse_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader_module,
                entry_point: "vs_main",
                buffers: &[BasicVertex::vertex_layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: render_resources.surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self { render_pipeline }
    }
}

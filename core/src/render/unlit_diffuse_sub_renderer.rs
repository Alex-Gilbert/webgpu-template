use bevy_ecs::{
    system::{Query, Res, SystemState},
    world::World,
};

use crate::{
    ecs::components::{
        gpu_bindings::model_bindings::ModelBindings,
        materials::unlit_diffuse_material::UnlitDiffuseMaterial, mesh_filter::BasicMeshFilter,
    },
    gpu_resources::pipelines::unlit_diffuse_pipeline::UnlitDiffusePipeline,
};

type UnlitDiffuseSubRendererSystemState = SystemState<(
    Res<'static, UnlitDiffusePipeline>,
    Query<
        'static,
        'static,
        (
            &'static ModelBindings,
            &'static BasicMeshFilter,
            &'static UnlitDiffuseMaterial,
        ),
    >,
)>;

pub struct UnlitDiffuseSubRenderer {
    pub system_state: UnlitDiffuseSubRendererSystemState,
}

impl UnlitDiffuseSubRenderer {
    pub fn new(world: &mut World) -> Self {
        Self {
            system_state: SystemState::new(world),
        }
    }

    pub fn render<'a, 'w>(&mut self, world: &'w World, render_pass: &mut wgpu::RenderPass<'a>)
    where
        'w: 'a,
    {
        let (pipeline, model_query) = self.system_state.get(world);

        render_pass.set_pipeline(&pipeline.into_inner().render_pipeline);
        for (model_binding, mesh_filter, material) in model_query.iter_inner() {
            render_pass.set_bind_group(1, &model_binding.bind_group, &[]);
            render_pass.set_bind_group(2, &material.bind_group, &[]);

            mesh_filter.filter.draw(render_pass);
        }
    }
}

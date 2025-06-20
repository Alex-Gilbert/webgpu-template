use bevy_ecs::{
    system::{Query, Res, SystemState},
    world::World,
};

use crate::{
    asset_management::{asset_bank::AssetBank, asset_handle::AssetHandle},
    ecs::components::{gpu_bindings::model_bindings::ModelBindings, mesh_filter::BasicMeshFilter},
    gpu_resources::pipelines::unlit_diffuse_pipeline::UnlitDiffusePipeline,
    materials::unlit_diffuse_material::UnlitDiffuseMaterial,
};

type UnlitDiffuseSubRendererSystemState = SystemState<(
    Res<'static, UnlitDiffusePipeline>,
    Res<'static, AssetBank<UnlitDiffuseMaterial>>,
    Query<
        'static,
        'static,
        (
            &'static ModelBindings,
            &'static BasicMeshFilter,
            &'static AssetHandle<UnlitDiffuseMaterial>,
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
        let (pipeline, unlit_material_bank, model_query) = self.system_state.get(world);

        // Note to my future self... `into_inner` here is extremely important
        // this allows us to get a reference to the resource with the world's lifetime
        // the world's lifetime 'w is set to live longer than the render_pass 'a
        // A similar thing is happening with the model_query ... see iter_inner()
        let pipeline = pipeline.into_inner();
        let unlit_material_bank = unlit_material_bank.into_inner();

        render_pass.set_pipeline(&pipeline.render_pipeline);
        for (model_binding, mesh_filter, material) in model_query.iter_inner() {
            let material = unlit_material_bank.get_asset(&material);

            if let Some(material) = material {
                render_pass.set_bind_group(1, &model_binding.bind_group, &[]);
                render_pass.set_bind_group(2, &material.texture_bind_group, &[]);
                mesh_filter.filter.draw(render_pass);
            }
        }
    }
}

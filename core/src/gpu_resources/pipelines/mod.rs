use bevy_ecs::world::World;

pub mod unlit_diffuse_pipeline;

pub fn initialize_pipelines(world: &mut World) {
    let unlit_diffuse_pipeline = unlit_diffuse_pipeline::UnlitDiffusePipeline::new(world);

    world.insert_resource(unlit_diffuse_pipeline);
}

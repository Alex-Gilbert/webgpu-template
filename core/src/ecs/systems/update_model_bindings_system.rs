use bevy_ecs::system::{Query, Res};

use crate::{
    ecs::components::{gpu_bindings::model_bindings::ModelBindings, transform::Transform},
    gpu_resources::render_resources::RenderResources,
};

pub fn update_model_bindings_system(
    render_resources: Res<RenderResources>,
    mut model_query: Query<(&mut Transform, &mut ModelBindings)>,
) {
    let queue = &render_resources.queue;

    for (mut transform, mut bindings) in model_query.iter_mut() {
        bindings.update(&render_resources.queue, transform.into_inner());
    }
}

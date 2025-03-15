use bevy_ecs::system::{Query, Res};

use crate::{
    ecs::{
        components::{
            camera::Camera, gpu_bindings::camera_bindings::CameraBindings, transform::Transform,
        },
        resources::screen_parameters::ScreenParameters,
    },
    gpu_resources::render_resources::RenderResources,
};

pub fn update_camera_system(
    screen_parameters: Res<ScreenParameters>,
    mut camera_query: Query<(&mut Camera,)>,
) {
    let (mut camera,) = camera_query.single_mut();
    let aspect_ratio = (screen_parameters.width as f64 / screen_parameters.height as f64) as f32;
    camera.set_aspect_ratio(aspect_ratio);
}

pub fn update_camera_bindings(
    render_resources: Res<RenderResources>,
    mut camera_query: Query<(&mut Camera, &mut Transform, &mut CameraBindings)>,
) {
    let (camera, transform, mut bindings) = camera_query.single_mut();
    bindings.update(
        &render_resources.queue,
        camera.into_inner(),
        transform.into_inner(),
    );
}

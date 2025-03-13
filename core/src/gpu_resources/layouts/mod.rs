use bevy_ecs::world::World;

pub mod camera_uniform_layout;
pub mod model_uniform_layout;
pub mod texture_uniform_layout;

pub fn initialize_bind_group_layouts(world: &mut World, device: &wgpu::Device) {
    // Initialize camera uniform bind group layout and insert it into the world
    world.insert_resource(camera_uniform_layout::CameraUniformLayout::new(device));

    world.insert_resource(model_uniform_layout::ModelUniformLayout::new(device));

    // Initialize texture uniform bind group layout and insert it into the world
    world.insert_resource(texture_uniform_layout::TextureBindGroupLayout::<1>::new(
        device,
    ));
    world.insert_resource(texture_uniform_layout::TextureBindGroupLayout::<2>::new(
        device,
    ));
    world.insert_resource(texture_uniform_layout::TextureBindGroupLayout::<3>::new(
        device,
    ));
    world.insert_resource(texture_uniform_layout::TextureBindGroupLayout::<4>::new(
        device,
    ));
}

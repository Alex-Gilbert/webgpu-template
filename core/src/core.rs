use std::sync::Arc;

use bevy_ecs::{schedule::Schedule, world::World};
use glam::vec3;
use log::trace;
use wgpu::{CommandBuffer, TextureFormat};

use crate::{
    ecs::{
        components::{
            gpu_bindings::model_bindings::ModelBindings,
            materials::unlit_diffuse_material::UnlitDiffuseMaterial, transform::Transform,
        },
        entity_bundles::camera_bundle::CameraBundle,
        resources::{
            apc_resources::{ApcPlatform, ApcQueue},
            http_resources::HttpPlatform,
            input::Input,
            screen_parameters::ScreenParameters,
            time::Time,
        },
        systems::{
            update_camera_system::{update_camera_bindings, update_camera_system},
            update_input_system::update_input_system,
            update_model_bindings_system::update_model_bindings_system,
        },
    },
    gpu_resources, include_texture,
    render::root_renderer::RootRenderer,
    traits::{apc_traits::ApcHandler, http_traits::HttpRequester},
    utils::primitives,
};

pub struct Core {
    pub world: World,
    early_update_schedule: Schedule,
    update_schedule: Schedule,
    late_update_schedule: Schedule,
    pre_render_schedule: Schedule,
    root_renderer: RootRenderer,
}

impl std::fmt::Debug for Core {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SkysharkCore").finish()
    }
}

impl Core {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        apc_handler: Arc<dyn ApcHandler>,
        http_requester: Arc<dyn HttpRequester>,
        render_width: u32,
        render_height: u32,
        texture_format: TextureFormat,
    ) -> Self {
        let mut world = World::new();
        gpu_resources::initialize_gpu_resources(
            &mut world,
            device.clone(),
            queue.clone(),
            texture_format,
        );

        world.insert_resource(Input::new());
        world.insert_resource(Time::new());
        world.insert_resource(ScreenParameters::new(render_width, render_height));
        world.insert_resource(ApcQueue::new());
        world.insert_resource(ApcPlatform {
            platform: apc_handler,
        });
        world.insert_resource(HttpPlatform {
            requester: http_requester,
        });

        let camera_bundle = CameraBundle::new(
            &world,
            vec3(0.0, 0.0, -10.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );

        world.spawn(camera_bundle);
        let root_renderer = RootRenderer::new(&mut world, render_width, render_height);

        // spawn a cube
        let texture = include_texture!("assets/textures/handsome.jpg", &device, &queue);

        let mut cube_transform = Transform::from_translation(vec3(0.0, 0.0, 0.0));
        let cube_mesh_filter = primitives::create_cube(&device, 3.0, 1);
        let cube_model_bindings = ModelBindings::new(&world, &device, &mut cube_transform);
        let cube_material = UnlitDiffuseMaterial::new(&world, &texture);

        world.spawn((
            cube_transform,
            cube_mesh_filter,
            cube_model_bindings,
            cube_material,
        ));

        let mut early_update_schedule = Schedule::default();
        let update_schedule = Schedule::default();
        let mut late_update_schedule = Schedule::default();
        let mut pre_render_schedule = Schedule::default();

        early_update_schedule.add_systems(update_camera_system);

        late_update_schedule.add_systems(update_input_system);

        pre_render_schedule.add_systems(update_camera_bindings);
        pre_render_schedule.add_systems(update_model_bindings_system);

        Self {
            world,
            early_update_schedule,
            update_schedule,
            late_update_schedule,
            pre_render_schedule,
            root_renderer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        trace!("resize");

        self.world
            .get_resource_mut::<ScreenParameters>()
            .unwrap()
            .set_size(width, height);

        let render_resources = &self
            .world
            .get_resource::<crate::gpu_resources::render_resources::RenderResources>()
            .unwrap();

        let device = &render_resources.device;

        self.root_renderer.set_size(device, width, height);
    }

    pub fn update(&mut self, delta_time: f32) {
        trace!("update");
        self.world
            .get_resource_mut::<Time>()
            .unwrap()
            .new_frame(delta_time);

        // check for completed apcs
        self.world
            .resource_scope(|world, apc_queue: bevy_ecs::world::Mut<ApcQueue>| {
                // Drain all available APC callbacks.
                while let Ok(callback) = apc_queue.receiver.try_recv() {
                    // Execute the callback, which can modify the SkysharkCore.
                    callback(world);
                }
            });

        // run the schedules
        self.early_update_schedule.run(&mut self.world);
        self.update_schedule.run(&mut self.world);
        self.late_update_schedule.run(&mut self.world);
    }

    /// Render the current state of the World
    /// This returns the command buffer filled with the commands to
    /// render the current state into the given texture view
    pub fn render(&mut self, texture_view: &wgpu::TextureView) -> CommandBuffer {
        trace!("render");
        self.pre_render_schedule.run(&mut self.world);
        self.root_renderer.render(&self.world, texture_view)
    }

    pub fn key_down(&mut self, key_code: winit::keyboard::KeyCode) {
        self.world
            .get_resource_mut::<Input>()
            .unwrap()
            .keyboard
            .get_or_insert_key(key_code)
            .press();
    }

    pub fn key_up(&mut self, key_code: winit::keyboard::KeyCode) {
        self.world
            .get_resource_mut::<Input>()
            .unwrap()
            .keyboard
            .get_or_insert_key(key_code)
            .release();
    }

    pub fn mouse_move(&mut self, x: f64, y: f64) {
        self.world
            .get_resource_mut::<Input>()
            .unwrap()
            .mouse
            .set_position(x, y);
    }

    pub fn mouse_button_down(&mut self, button: winit::event::MouseButton) {
        self.world
            .get_resource_mut::<Input>()
            .unwrap()
            .mouse
            .get_or_insert_button(button)
            .press();
    }

    pub fn mouse_up(&mut self, button: winit::event::MouseButton) {
        self.world
            .get_resource_mut::<Input>()
            .unwrap()
            .mouse
            .get_or_insert_button(button)
            .release();
    }

    pub fn mouse_scroll(&mut self, delta_x: f64, delta_y: f64) {
        self.world
            .get_resource_mut::<Input>()
            .unwrap()
            .mouse
            .set_scroll(delta_x, delta_y);
    }

    pub fn get_root_renderer(&self) -> &RootRenderer {
        &self.root_renderer
    }

    pub fn get_root_renderer_mut(&mut self) -> &mut RootRenderer {
        &mut self.root_renderer
    }
}

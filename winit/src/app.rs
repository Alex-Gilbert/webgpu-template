use std::sync::Arc;

use demo_core::{
    core::Core,
    traits::{apc_traits::ApcHandler, http_traits::HttpRequester},
};
use log::info;
use wgpu::TextureFormat;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ControlFlow,
    keyboard::{Key, KeyCode, NamedKey, PhysicalKey},
};

#[cfg(feature = "debug-renderdoc")]
use renderdoc::{RenderDoc, V141};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

use crate::{traits::DemoWinitHandler, user_event::DemoWinitEvent};

#[derive(Debug)]
struct DemoWinitAppUninit<H> {
    demo_handler: H,
}

#[derive(Debug)]
struct DemoWinitAppInit<H> {
    window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    demo_handler: H,

    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,

    target_buffer_width: u32,
    target_buffer_height: u32,

    time_of_last_update: Instant,

    #[cfg(target_arch = "wasm32")]
    frame_count: u32,

    #[cfg(feature = "debug-renderdoc")]
    renderdoc: RenderDoc<V141>,

    pub demo_core: Core,
}

/// The main application struct for a demo winit application.
#[derive(Debug)]
pub struct DemoWinitApp<H> {
    inner: DemoWinitAppInner<H>,
}

#[derive(Default, Debug)]
enum DemoWinitAppInner<H> {
    Uninit(DemoWinitAppUninit<H>),
    ReadyToInit(DemoWinitAppUninit<H>),
    Init(DemoWinitAppInit<H>),

    #[default]
    Dummy,
}

impl<H> DemoWinitApp<H> {
    /// Create a new demo winit application.
    pub fn new(demo_winit_handler: H) -> Self {
        Self {
            inner: DemoWinitAppInner::Uninit(DemoWinitAppUninit {
                demo_handler: demo_winit_handler,
            }),
        }
    }

    fn ready_init(&mut self) {
        let uninit = match core::mem::take(&mut self.inner) {
            DemoWinitAppInner::Uninit(uninit) => uninit,
            _ => panic!("Tried to initialize demo twice"),
        };

        self.inner = DemoWinitAppInner::ReadyToInit(uninit);
    }

    fn assume_init(&mut self) -> &mut DemoWinitAppInit<H> {
        match &mut self.inner {
            DemoWinitAppInner::Init(init) => init,
            _ => panic!("Demo not initialized"),
        }
    }
}

impl<H: DemoWinitHandler> DemoWinitApp<H> {
    fn init_if_ready(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut uninit = match &self.inner {
            DemoWinitAppInner::ReadyToInit(_) => match core::mem::take(&mut self.inner) {
                DemoWinitAppInner::ReadyToInit(uninit) => uninit,
                _ => unreachable!(),
            },
            _ => return,
        };

        // build window
        let window = uninit.demo_handler.build_window(event_loop).unwrap();
        let window = Arc::new(window);

        // cerate instance
        let instance = uninit.demo_handler.create_instance();

        // create surface
        let surface = H::create_surface(&instance, &window);

        //pick adapter
        let adapter = H::select_adapter(&instance, Some(&surface));
        let info = adapter.get_info();
        println!("Adapter: {} ({:?})", info.name, info.backend);

        // get the device and queue
        let (device, queue) = uninit.demo_handler.request_device(&adapter);

        // cinfigur surface
        let size = window.inner_size();
        let surface_config = H::configure_surface(&surface, &adapter, size);

        surface.configure(&device, &surface_config);

        let physical_size = window.inner_size();

        #[cfg(not(target_arch = "wasm32"))]
        let target_buffer_width = physical_size.width;
        #[cfg(not(target_arch = "wasm32"))]
        let target_buffer_height = physical_size.height;

        #[cfg(target_arch = "wasm32")]
        let target_buffer_width = physical_size.width.clamp(1, 2048);
        #[cfg(target_arch = "wasm32")]
        let target_buffer_height = physical_size.height.clamp(1, 2048);

        info!("width: {}", target_buffer_width);
        info!("height: {}", target_buffer_height);

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let apc_handler = Arc::<dyn ApcHandler>::from(H::build_apc_handler());
        let http_requester = Arc::<dyn HttpRequester>::from(H::build_http_requester());

        let demo_core = Core::new(
            device.clone(),
            queue.clone(),
            apc_handler.clone(),
            http_requester.clone(),
            target_buffer_width,
            target_buffer_height,
            surface_config.format,
        );

        let init = DemoWinitAppInit {
            window,
            surface,
            surface_config,
            demo_handler: uninit.demo_handler,
            device,
            queue,
            target_buffer_width,
            target_buffer_height,
            demo_core,
            #[cfg(target_arch = "wasm32")]
            frame_count: 0,
            time_of_last_update: Instant::now(),

            #[cfg(feature = "debug-renderdoc")]
            renderdoc: RenderDoc::<V141>::new().expect("Failed to initialize RenderDoc"),
        };

        self.inner = DemoWinitAppInner::Init(init);
    }
}

impl<H> DemoWinitAppInit<H> {
    #[cfg(target_arch = "wasm32")]
    fn resize_surface_if_needed(
        target_buffer_width: &mut u32,
        target_buffer_height: &mut u32,
        surface: &wgpu::Surface<'static>,
        device: &Arc<wgpu::Device>,
        surface_config: &mut wgpu::SurfaceConfiguration,
        frame_count: &u32,
    ) {
        let (width_delta, height_delta) = {
            let current_buffer_width = surface_config.width;
            let current_buffer_height = surface_config.height;
            (
                (*target_buffer_width as i32 - current_buffer_width as i32).abs(),
                (*target_buffer_height as i32 - current_buffer_height as i32).abs(),
            )
        };

        if width_delta > 100
            || height_delta > 100
            || (width_delta + height_delta > 0 && frame_count % 10 == 0)
        {
            Self::resize_surface(
                target_buffer_width,
                target_buffer_height,
                surface,
                device,
                surface_config,
            );
        }
    }

    fn resize_surface(
        target_buffer_width: &mut u32,
        target_buffer_height: &mut u32,
        surface: &wgpu::Surface<'static>,
        device: &Arc<wgpu::Device>,
        surface_config: &mut wgpu::SurfaceConfiguration,
    ) {
        if *target_buffer_width != surface_config.width
            || *target_buffer_height != surface_config.height
        {
            surface_config.width = *target_buffer_width;
            surface_config.height = *target_buffer_height;
            surface.configure(device, surface_config);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn resize(
        physical_size: winit::dpi::PhysicalSize<u32>,
        target_buffer_width: &mut u32,
        target_buffer_height: &mut u32,
        surface: &wgpu::Surface<'static>,
        device: &Arc<wgpu::Device>,
        surface_config: &mut wgpu::SurfaceConfiguration,
    ) {
        let width = std::cmp::max(1, physical_size.width);
        let height = std::cmp::max(1, physical_size.height);

        *target_buffer_width = width;
        *target_buffer_height = height;

        Self::resize_surface(
            target_buffer_width,
            target_buffer_height,
            surface,
            device,
            surface_config,
        );
    }

    #[cfg(target_arch = "wasm32")]
    pub fn resize(
        physical_size: winit::dpi::PhysicalSize<u32>,
        target_buffer_width: &mut u32,
        target_buffer_height: &mut u32,
    ) {
        let actual_width = std::cmp::max(1, physical_size.width);
        let actual_height = std::cmp::max(1, physical_size.height);

        let target_width = std::cmp::min(2048, actual_width);
        let target_height = std::cmp::min(2048, actual_height);

        *target_buffer_width = target_width;
        *target_buffer_height = target_height;
    }

    pub fn render_and_present(&mut self) {
        // get the surface texture and texture view for the render pass
        let surface_texture = self.surface.get_current_texture();
        if surface_texture.is_err() {
            // TODO: Handle this error
            // we need to be able to rebuild the surface if it's lost
            return;
        }
        let surface_texture = surface_texture.unwrap();
        let texture_view = surface_texture.texture.create_view(&Default::default());
        let command_buffer = self.demo_core.render(&texture_view);

        let _ = &self.queue.submit(std::iter::once(command_buffer));

        self.window.pre_present_notify();
        surface_texture.present();
    }
}

impl<H: DemoWinitHandler + 'static> ApplicationHandler<DemoWinitEvent> for DemoWinitApp<H> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.init_if_ready(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let demo_winit = self.assume_init();
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key,
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        repeat: false,
                        ..
                    },
                ..
            } => match state {
                ElementState::Pressed => {
                    #[cfg(feature = "debug-renderdoc")]
                    match key_code {
                        KeyCode::F10 => demo_winit.renderdoc.trigger_capture(),
                        // KeyCode::F8 => demo_winit.renderdoc.start(),
                        // KeyCode::F9 => demo_winit.renderdoc.end_capture(),
                        _ => demo_winit.demo_core.key_down(key_code),
                    }

                    #[cfg(not(feature = "debug-renderdoc"))]
                    demo_winit.demo_core.key_down(key_code)
                }
                ElementState::Released => {
                    match logical_key.as_ref() {
                        Key::Named(NamedKey::Escape) => {
                            // do other cleanup here
                            event_loop.exit();
                        }
                        _ => demo_winit.demo_core.key_up(key_code),
                    }
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                demo_winit.demo_core.mouse_move(position.x, position.y);
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => demo_winit.demo_core.mouse_button_down(button),
                ElementState::Released => demo_winit.demo_core.mouse_up(button),
            },
            WindowEvent::MouseWheel { delta, .. } => {
                let pixels_per_line = 38.0;
                match delta {
                    winit::event::MouseScrollDelta::PixelDelta(delta) => {
                        demo_winit
                            .demo_core
                            .mouse_scroll(0.0, delta.y.signum() * pixels_per_line);
                    }
                    winit::event::MouseScrollDelta::LineDelta(_x, y) => {
                        demo_winit
                            .demo_core
                            .mouse_scroll(0.0, y.signum() as f64 * pixels_per_line);
                    }
                }
            }
            WindowEvent::Resized(..) | WindowEvent::ScaleFactorChanged { .. } => {
                let physical_size = demo_winit.window.inner_size();

                #[cfg(not(target_arch = "wasm32"))]
                DemoWinitAppInit::<H>::resize(
                    physical_size,
                    &mut demo_winit.target_buffer_width,
                    &mut demo_winit.target_buffer_height,
                    &demo_winit.surface,
                    &demo_winit.device,
                    &mut demo_winit.surface_config,
                );

                #[cfg(target_arch = "wasm32")]
                DemoWinitAppInit::<H>::resize(
                    physical_size,
                    &mut demo_winit.target_buffer_width,
                    &mut demo_winit.target_buffer_height,
                );

                demo_winit
                    .demo_core
                    .resize(physical_size.width, physical_size.height);

                demo_winit.window.request_redraw();
            }
            WindowEvent::CloseRequested => {
                // do other cleanup here
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                demo_winit.demo_handler.on_pre_draw();
                demo_winit.render_and_present();
                demo_winit.demo_handler.on_post_draw();
            }
            _ => (),
        }
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        match cause {
            winit::event::StartCause::Init => {
                self.ready_init();
                event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now()));
            }
            winit::event::StartCause::ResumeTimeReached { .. } => {
                let demo_winit = self.assume_init();
                let now = Instant::now();
                demo_winit.demo_handler.on_pre_update();
                demo_winit
                    .demo_core
                    .update((now - demo_winit.time_of_last_update).as_secs_f32());

                #[cfg(target_arch = "wasm32")]
                DemoWinitAppInit::<H>::resize_surface_if_needed(
                    &mut demo_winit.target_buffer_width,
                    &mut demo_winit.target_buffer_height,
                    &demo_winit.surface,
                    &demo_winit.device,
                    &mut demo_winit.surface_config,
                    &demo_winit.frame_count,
                );

                demo_winit.demo_handler.on_post_update();

                #[cfg(target_arch = "wasm32")]
                {
                    demo_winit.frame_count += 1;
                }

                demo_winit.time_of_last_update = now;
                demo_winit.window.request_redraw();

                event_loop.set_control_flow(ControlFlow::WaitUntil(
                    now + Duration::from_millis(1000 / 60),
                ));
            }
            winit::event::StartCause::WaitCancelled { .. } => {
                let _ = event_loop;
            }
            winit::event::StartCause::Poll => {
                unreachable!()
            }
        }
    }

    fn user_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: DemoWinitEvent,
    ) {
        let _demo_winit = self.assume_init();
        match event {
            DemoWinitEvent::Kill => {
                #[cfg(target_arch = "wasm32")]
                event_loop.exit();
                #[cfg(not(target_arch = "wasm32"))]
                let _ = event_loop;
            }
        }
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let demo_winit = self.assume_init();
        demo_winit.demo_handler.on_exit();
    }
}

use std::sync::Arc;

use demo_core::traits::{apc_traits::ApcHandler, http_traits::HttpRequester};
use winit::{event_loop::ActiveEventLoop, window::Window};

/// A trait for configuring our winit window.
pub trait DemoWinitHandler {
    /// Construct the window from the active event loop.
    fn build_window(&mut self, event_loop: &ActiveEventLoop) -> Result<Window, String>;

    /// Create an apc handler.
    fn build_apc_handler() -> Box<dyn ApcHandler>;

    /// Create an http requester.
    fn build_http_requester() -> Box<dyn HttpRequester>;

    /// Window cleanup.
    fn on_exit(&self) {}

    /// Called before the demo core is updated.
    fn on_pre_update(&self) {}

    /// Called after demo core is updated.
    fn on_post_update(&self) {}

    /// Called before demo core is rendered.
    fn on_pre_draw(&self) {}

    /// Called after demo core is rendered.
    fn on_post_draw(&self) {}

    fn create_instance(&self) -> wgpu::Instance {
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            #[cfg(not(feature = "debug-renderdoc"))]
            flags: wgpu::InstanceFlags::DEBUG | wgpu::InstanceFlags::VALIDATION,
            #[cfg(feature = "debug-renderdoc")]
            flags: wgpu::InstanceFlags::empty(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        })
    }

    /// Given your instance and window, build a surface
    fn create_surface<'a>(
        instance: &wgpu::Instance,
        window: &Arc<winit::window::Window>,
    ) -> wgpu::Surface<'a> {
        instance.create_surface(window.clone()).unwrap()
    }

    /// Given instance & surface, pick your adapter
    fn select_adapter(instance: &wgpu::Instance, surface: Option<&wgpu::Surface>) -> wgpu::Adapter {
        // default: try high perf → low perf → headless → fallback
        let mut opts = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: surface,
            force_fallback_adapter: false,
        };

        if let Some(a) = futures::executor::block_on(instance.request_adapter(&opts)) {
            return a;
        }
        println!("Failed To find high-performance adapter, trying low-power...");
        opts.power_preference = wgpu::PowerPreference::LowPower;
        if let Some(a) = futures::executor::block_on(instance.request_adapter(&opts)) {
            return a;
        }
        println!("Failed To find adapter with surface compatibility, trying headless mode...");
        opts.compatible_surface = None;
        if let Some(a) = futures::executor::block_on(instance.request_adapter(&opts)) {
            return a;
        }
        println!("No hardware adapters found, trying fallback adapter");
        opts.force_fallback_adapter = true;
        if let Some(a) = futures::executor::block_on(instance.request_adapter(&opts)) {
            return a;
        }

        eprintln!("ERROR: no adapter found, exiting");
        std::process::exit(1);
    }

    /// Given your adapter, spin up device + queue
    fn request_device(&self, adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: Default::default(),
                required_limits: wgpu::Limits {
                    #[cfg(not(target_arch = "wasm32"))]
                    max_texture_dimension_2d: 8192,
                    ..wgpu::Limits::downlevel_webgl2_defaults()
                },
            },
            None,
        ))
        .unwrap()
    }

    /// Given the surface, device, queue, and window size, produce a `SurfaceConfiguration`
    fn configure_surface(
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> wgpu::SurfaceConfiguration {
        let caps = surface.get_capabilities(adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap_or(caps.formats[0]);
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.clamp(1, 8192),
            height: size.height.clamp(1, 8192),
            present_mode: caps.present_modes[0],
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }
}

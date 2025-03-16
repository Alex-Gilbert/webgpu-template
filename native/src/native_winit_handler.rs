use demo_core::traits::{apc_traits::ApcHandler, http_traits::HttpRequester};

use demo_winit::traits::DemoWinitHandler;
use winit::{dpi::LogicalSize, window::WindowAttributes};

use crate::{native_apc_handler::NativeApcHandler, native_http_requester::NativeHttpRequester};

// Struct to hold the clients list and implement the callback
pub struct NativeWinitHandler {}

impl DemoWinitHandler for NativeWinitHandler {
    fn build_window(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
    ) -> Result<winit::window::Window, String> {
        let attributes = WindowAttributes::default()
            .with_title("Skyshark")
            .with_inner_size(LogicalSize::new(800.0, 600.0));

        let window = event_loop
            .create_window(attributes)
            .map_err(|e| format!("Failed to create window: {}", e))?;

        Ok(window)
    }

    fn build_apc_handler() -> Box<dyn ApcHandler> {
        Box::new(NativeApcHandler)
    }

    fn build_http_requester() -> Box<dyn HttpRequester> {
        Box::new(NativeHttpRequester)
    }
}

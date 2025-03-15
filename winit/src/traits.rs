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
}

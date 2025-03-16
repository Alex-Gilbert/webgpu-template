use std::{collections::HashMap, sync::Arc};

use demo_native::native_winit_handler::NativeWinitHandler;
use demo_winit::{app::DemoWinitApp, user_event::DemoWinitEvent};
use log::{info, warn};
use winit::event_loop::EventLoop;

fn main() -> Result<(), String> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();

    let winit_handler = NativeWinitHandler {};

    let mut app = DemoWinitApp::new(winit_handler);

    let event_loop = EventLoop::<DemoWinitEvent>::with_user_event()
        .build()
        .map_err(|e| format!("Failed to create event loop: {}", e))?;

    event_loop
        .run_app(&mut app)
        .map_err(|e| format!("Failed to run event loop: {}", e))?;

    Ok(())
}

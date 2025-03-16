use crossbeam::channel::Sender;
use demo_core::traits::apc_traits::{Apc, ApcCallback, ApcHandler};
use tokio::runtime::Runtime;

pub struct NativeApcHandler;

impl ApcHandler for NativeApcHandler {
    fn spawn_apc(&self, apc: Apc, sender: Sender<ApcCallback>) {
        // Spawn a new thread to run the APC's future.
        std::thread::spawn(move || {
            // Create a new Tokio runtime.
            let rt = Runtime::new().expect("Failed to create Tokio runtime");

            // Block on the APC's future.
            let callback = rt.block_on(apc.future);

            // Send the callback to via the sender.
            sender
                .send(callback)
                .expect("Failed to enqueue APC callback");
        });
    }
}

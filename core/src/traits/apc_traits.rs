use bevy_ecs::world::World;
use crossbeam::channel::Sender;
use std::{future::Future, pin::Pin};

/// The callback that will be executed when an APC completes.
/// It receives a mutable reference to Skyshark Core's bevy_ecs::World.
pub type ApcCallback = Box<dyn FnOnce(&mut World) + Send>;

/// An asynchronous procedure call task. When its future completes,
/// it will yield an APCCallback.
pub struct Apc {
    pub future: Pin<Box<dyn Future<Output = ApcCallback> + Send>>,
}

/// Our unified trait for asynchronous operations.
/// Consumers of skyshark core can implement their own platform to run these tasks.
pub trait ApcHandler: Send + Sync {
    /// Spawns an asynchronous procedure call.
    fn spawn_apc(&self, apc: Apc, sender: Sender<ApcCallback>);
}

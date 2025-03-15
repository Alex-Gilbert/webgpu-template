use std::sync::Arc;

use bevy_ecs::system::Resource;
use crossbeam::channel::{Receiver, Sender, unbounded};

use crate::traits::apc_traits::{ApcCallback, ApcHandler};

/// A resource that holds a channel for completed APC callbacks.
/// APC tasks will send their callback here when done.
/// This uses crossbeam's unbounded channel meaning it is shared between threads and can be used
/// as a resource in Bevy ECS.
#[derive(Resource)]
pub struct ApcQueue {
    pub sender: Sender<ApcCallback>,
    pub receiver: Receiver<ApcCallback>,
}

impl ApcQueue {
    /// Create a new APCQueue.
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }
}

/// A resource that holds the APC platform.
/// An Apc(Asynchronous Procedure Call) is a task that will be executed asynchronously.
/// Consumers of skyshark core can implement their own platform to run these tasks.
#[derive(Resource)]
pub struct ApcPlatform {
    pub platform: Arc<dyn ApcHandler>,
}

impl ApcPlatform {
    pub fn new(platform: Arc<dyn ApcHandler>) -> Self {
        Self {
            platform: platform.clone(),
        }
    }
}

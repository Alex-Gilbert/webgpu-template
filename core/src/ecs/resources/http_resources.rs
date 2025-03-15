use std::sync::Arc;

use bevy_ecs::system::Resource;

use crate::traits::http_traits::HttpRequester;

/// A resource that holds the HTTP requester.
#[derive(Resource)]
pub struct HttpPlatform {
    pub requester: Arc<dyn HttpRequester>,
}

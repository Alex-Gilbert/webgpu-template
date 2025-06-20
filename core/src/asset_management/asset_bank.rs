use bevy_ecs::system::Resource;

use super::asset_handle::AssetHandle;

/// A store of assets, this is meant to be a resource in the ECS
#[derive(Resource)]
pub struct AssetBank<T> {
    assets: Vec<T>,
    next_id: usize,
}

impl<T> Default for AssetBank<T> {
    fn default() -> Self {
        Self {
            assets: Vec::new(),
            next_id: 0,
        }
    }
}

impl<T> AssetBank<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an asset and get a handle to it
    pub fn add(&mut self, asset: T) -> AssetHandle<T> {
        let id = self.next_id;
        self.next_id += 1;
        self.assets.push(asset);
        AssetHandle::new(id)
    }

    /// Get asset by handle (immutable)
    pub fn get_asset(&self, handle: &AssetHandle<T>) -> Option<&T> {
        self.assets.get(handle.id())
    }

    /// Get asset by handle (mutable)
    pub fn get_asset_mut(&mut self, handle: &AssetHandle<T>) -> Option<&mut T> {
        self.assets.get_mut(handle.id())
    }
}

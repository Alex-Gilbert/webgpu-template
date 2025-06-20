use std::collections::HashMap;

use bevy_ecs::system::Resource;

use super::asset_handle::AssetHandle;

#[derive(Resource, Default)]
pub struct NamedAssets<T> {
    names: HashMap<String, AssetHandle<T>>,
}

impl<T> NamedAssets<T> {
    /// Insert (or overwrite) the mapping `name -> handle`
    pub fn insert(&mut self, name: impl Into<String>, handle: AssetHandle<T>) {
        self.names.insert(name.into(), handle);
    }

    /// Look up the handle by name
    pub fn get(&self, name: &str) -> Option<AssetHandle<T>> {
        self.names.get(name).cloned()
    }

    /// Remove a name mapping (does *not* drop the asset)
    pub fn remove(&mut self, name: &str) -> Option<AssetHandle<T>> {
        self.names.remove(name)
    }

    /// Check if a name is registered
    pub fn contains(&self, name: &str) -> bool {
        self.names.contains_key(name)
    }

    /// Get a collection of handles by their named pattern
    pub fn get_pattern(&self, prefix: &str, count: usize) -> Vec<AssetHandle<T>> {
        (0..count)
            .filter_map(|i| {
                let name = format!("{}_{}", prefix, i);
                self.get(&name)
            })
            .collect()
    }
}

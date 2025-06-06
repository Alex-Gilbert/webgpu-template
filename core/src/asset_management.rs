use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy_ecs::system::Resource;

#[derive(Debug, Copy, Eq)]
pub struct Handle<T> {
    id: usize,
    _phantom: PhantomData<T>,
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            id: self.id,
            _phantom: self._phantom,
        }
    }
}

impl<T> Handle<T> {
    fn new(id: usize) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// A collection of assets
#[derive(Resource)]
pub struct Assets<T> {
    assets: Vec<T>,
    next_id: usize,
}

impl<T> Default for Assets<T> {
    fn default() -> Self {
        Self {
            assets: Vec::new(),
            next_id: 0,
        }
    }
}

impl<T> Assets<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an asset and get a handle to it
    pub fn add(&mut self, asset: T) -> Handle<T> {
        let id = self.next_id;
        self.next_id += 1;
        self.assets.push(asset);
        Handle::new(id)
    }

    /// Get asset by handle (immutable)
    pub fn get(&self, handle: &Handle<T>) -> Option<&T> {
        self.assets.get(handle.id())
    }

    /// Get asset by handle (mutable)
    pub fn get_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        self.assets.get_mut(handle.id())
    }

    /// Get all handles (simple range)
    pub fn handles(&self) -> impl Iterator<Item = Handle<T>> {
        (0..self.assets.len()).map(|i| Handle::new(i))
    }

    /// Check if handle is valid (simple bounds check)
    pub fn contains(&self, handle: &Handle<T>) -> bool {
        (handle.id()) < self.assets.len()
    }

    /// Number of assets stored
    pub fn len(&self) -> usize {
        self.assets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }

    /// Iterate over all assets with their handles
    pub fn iter(&self) -> impl Iterator<Item = (Handle<T>, &T)> {
        self.assets
            .iter()
            .enumerate()
            .map(|(i, asset)| (Handle::new(i), asset))
    }

    /// Reserve capacity for known number of assets
    pub fn reserve(&mut self, additional: usize) {
        self.assets.reserve(additional);
    }

    /// Pre-allocate exact capacity (useful for batch loading)
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            assets: Vec::with_capacity(capacity),
            next_id: 0,
        }
    }
}

#[derive(Resource, Default)]
pub struct NamedAssets<T> {
    names: HashMap<String, Handle<T>>,
}

impl<T> NamedAssets<T> {
    /// Insert (or overwrite) the mapping `name -> handle`
    pub fn insert(&mut self, name: impl Into<String>, handle: Handle<T>) {
        self.names.insert(name.into(), handle);
    }

    /// Look up the handle by name
    pub fn get(&self, name: &str) -> Option<Handle<T>> {
        self.names.get(name).cloned()
    }

    /// Remove a name mapping (does *not* drop the asset)
    pub fn remove(&mut self, name: &str) -> Option<Handle<T>> {
        self.names.remove(name)
    }

    /// Check if a name is registered
    pub fn contains(&self, name: &str) -> bool {
        self.names.contains_key(name)
    }
}

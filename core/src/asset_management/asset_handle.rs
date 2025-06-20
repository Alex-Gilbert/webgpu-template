use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy_ecs::component::Component;

#[derive(Component, Debug, Copy, Eq)]
pub struct AssetHandle<T> {
    id: usize,
    _phantom: PhantomData<T>,
}

impl<T> Hash for AssetHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialEq for AssetHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Clone for AssetHandle<T> {
    fn clone(&self) -> Self {
        AssetHandle {
            id: self.id,
            _phantom: self._phantom,
        }
    }
}

impl<T> AssetHandle<T> {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

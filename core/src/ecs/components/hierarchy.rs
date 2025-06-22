use bevy_ecs::{bundle::Bundle, component::Component, entity::Entity};
use glam::Mat4;

#[derive(Component, Debug, Clone, Default)]
pub struct WorldMatrix {
    pub parent_matrix: Mat4,
    pub local_matrix: Mat4,
    pub world_matrix: Mat4,
}

impl WorldMatrix {
    pub fn update_local_matrix(&mut self, new_local: Mat4) {
        self.local_matrix = new_local;
        self.world_matrix = self.parent_matrix * self.local_matrix;
    }

    pub fn update_local_and_parent_matrix(&mut self, new_local: Mat4, new_parent: Mat4) {
        self.local_matrix = new_local;
        self.parent_matrix = new_parent;
        self.world_matrix = self.parent_matrix * self.local_matrix;
    }
}

/// Component that marks an entity as having child entities
#[derive(Component, Debug, Clone, Default)]
pub struct Children {
    pub entities: Vec<Entity>,
}

impl Children {
    /// Creates a new Children component with no children
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new Children component with the given children
    pub fn with_children(entities: Vec<Entity>) -> Self {
        Self { entities }
    }

    /// Adds a child entity
    pub fn push(&mut self, entity: Entity) {
        if !self.entities.contains(&entity) {
            self.entities.push(entity);
        }
    }

    /// Removes a child entity
    pub fn remove(&mut self, entity: Entity) {
        if let Some(index) = self.entities.iter().position(|&e| e == entity) {
            self.entities.remove(index);
        }
    }

    /// Gets an iterator over the children
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    /// Gets the number of children
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Checks if there are no children
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

/// Component that marks an entity as having a parent entity
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parent {
    pub entity: Entity,
}

impl Parent {
    /// Creates a new Parent component
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }

    /// Gets the parent entity
    pub fn get(&self) -> Entity {
        self.entity
    }
}

/// A bundle for entities that need hierarchical transforms
#[derive(Bundle, Default)]
pub struct HierarchicalTransformBundle {
    pub local_transform: LocalTransform,
    pub global_transform: GlobalTransform,
}

impl HierarchicalTransformBundle {
    /// Creates a new bundle with identity transforms
    pub fn identity() -> Self {
        Self::default()
    }

    /// Creates a new bundle with the given local transform
    pub fn from_local_transform(local_transform: LocalTransform) -> Self {
        Self {
            local_transform,
            global_transform: Default::default(),
        }
    }
}

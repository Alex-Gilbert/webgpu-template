use bevy_ecs::{entity, prelude::*};
use glam::Mat4;

use crate::ecs::components::{
    hierarchy::{Children, Parent, WorldMatrix},
    transform::{self, Transform},
};

type RootComponents<'a> = (Entity, &'a Transform, &'a WorldMatrix, Option<&'a Children>);

type MightHaveChildren<'a> = (
    Entity,
    &'a Transform,
    &'a mut WorldMatrix,
    Option<&'a Children>,
);

type HasParent<'a> = (Entity, &'a Transform, &'a mut WorldMatrix, &'a Parent);
type HasBoth<'a> = (
    Entity,
    &'a Transform,
    &'a mut WorldMatrix,
    &'a Children,
    &'a Parent,
);

/// System that propagates transforms from parents to children
pub fn transform_propagation_system(
    root_query: Query<RootComponents, Without<Parent>>,
    mut changed_query: Query<MightHaveChildren, Changed<Transform>>,
    mut full_query: Query<MightHaveChildren>,
    child_query: Query<&Children>,
) {
    //The first step
    for (entity, _, _, _) in root_query.iter() {
        visit_entity(entity, &mut changed_query, &mut full_query, &child_query);
    }
}

fn visit_entity(
    entity: Entity,
    changed_query: &mut Query<MightHaveChildren, Changed<Transform>>,
    full_query: &mut Query<MightHaveChildren>,
    child_query: &Query<&Children>,
) {
    // Have I changed at all, if so... we update our matrix and propagate to the children
    if let Ok((_, transform, mut world_matrix, _)) = changed_query.get_mut(entity) {
        world_matrix.update_local_matrix(transform.get_matrix());

        // Propogate these changes to my children
        propagate_to_children(entity, &world_matrix.world_matrix, full_query, child_query);

        return;
    }

    // If we have not changed. We visit our children, if there are any
    if let Ok(children) = child_query.get(entity) {
        for child_entity in children.entities.iter() {
            visit_entity(*child_entity, changed_query, full_query, child_query);
        }
    }
}

/// Recursively propagates transforms to children
fn propagate_to_children(
    parent_entity: Entity,
    parent_world_matrix: &Mat4,
    full_query: &mut Query<MightHaveChildren>,
    child_query: &Query<&Children>,
) {
    if let Ok(children) = child_query.get(parent_entity) {
        for child_entity in children.entities.iter() {
            if let Ok((_, child_transform, mut child_world_matrix, _)) =
                full_query.get_mut(*child_entity)
            {
                child_world_matrix.update_local_and_parent_matrix(
                    child_transform.get_matrix(),
                    *parent_world_matrix,
                );

                let new_matrix = child_world_matrix.world_matrix;
                drop(child_world_matrix);

                propagate_to_children(*child_entity, &new_matrix, full_query, child_query);
            }
        }
    }
}

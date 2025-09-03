use bevy::{math::{bounding::{Aabb3d, BoundingVolume}}, prelude::*, render::primitives::Aabb};
use crate::components::{Bullet, Ground};

// Useful for bullets; could be made more generic with an associated AutoDespawn component ? 
pub(crate) fn despawn_out_of_world_bullets(
    mut commands: Commands,
    bullets_aabbs_query: Query<(Entity, &Aabb, &GlobalTransform), With<Bullet>>,
    ground_bounding_box_query: Query<&Aabb, With<Ground>>,
) {
    for ground_box in &ground_bounding_box_query {

        let center = ground_box.center;
        let mut half_size = ground_box.half_extents;
        half_size.y += 30.0; // arbitrary
        let ground_box_3d = Aabb3d::new(center, half_size);

        for (entity, bounding_box, global_transform) in &bullets_aabbs_query {
            // Check if entity is far outside the boundaries of the ground
            let bounding_box_3d = Aabb3d::new(bounding_box.center + global_transform.translation_vec3a(), bounding_box.half_extents);
            if !ground_box_3d.contains(&bounding_box_3d) {
                // here we should issue a command to despawn entity
                commands.entity(entity).despawn();
            }
        }
    }
}
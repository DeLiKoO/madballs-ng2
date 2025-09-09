use bevy::{math::{bounding::{Aabb3d, BoundingVolume}}, prelude::*, render::primitives::Aabb};
use crate::{components::Ground, weapon::components::Bullet};

// Useful for bullets; could be made more generic with an associated AutoDespawn component ? 
pub(crate) fn despawn_out_of_world_bullets(
    mut commands: Commands,
    bullets_aabbs_query: Query<(Entity, &Aabb, &GlobalTransform), With<Bullet>>,
    ground_bounding_box_query: Query<&Aabb, With<Ground>>,
) {
    for ground_box in &ground_bounding_box_query {

        const MAX_ALTITUDE: f32 = 30.0; // Maximum arbitrary altitude
        let ground_box_3d = Aabb3d::new(ground_box.center, ground_box.half_extents + MAX_ALTITUDE); 
        for (bullet_entity, bullet_box, bullet_global_transform) in &bullets_aabbs_query {
            // Check if entity is far outside the boundaries of the ground
            let bullet_box_3d = Aabb3d::new(bullet_global_transform.transform_point(bullet_box.center.into()), bullet_box.half_extents);
            if !ground_box_3d.contains(&bullet_box_3d) {
                // here we should issue a command to despawn entity
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}
use bevy::{math::{bounding::{Aabb3d, BoundingVolume}, Vec3A}, prelude::*, render::primitives::Aabb};

use crate::{components::*, weapon::components::Bullet};

pub(crate) fn handle_bullet_collision_aio(
    mut commands: Commands,
    mut with_health_query: Query<(&Aabb, &mut Health, &GlobalTransform), With<Health>>,
    bullets_query: Query<(Entity, &Aabb, &Damage, &GlobalTransform), With<Bullet>>,
) {
    for (with_health_box, mut health, with_health_global_transform) in &mut with_health_query {
        let with_health_box_3d = to_global_aabb3d(with_health_box, with_health_global_transform);
        for (bullet_entity, bullet_bounding_box, bullet_damage, bullet_global_transform) in &bullets_query {
            let bullet_box_3d = to_global_aabb3d(bullet_bounding_box, bullet_global_transform);
            if with_health_box_3d.contains(&bullet_box_3d) {
                // We detected a collision between Bullet and entity with Health
                health.points = health.points - bullet_damage.points;
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

// Apply global transform to local-bound Aabb of the Entity
// in order to return a proper Aabb3d bounding box in the world's coordinates
// NOTICE: Aabb3d is {min, max} whereas Aabb is {center, half_extents}
fn to_global_aabb3d(aabb: &Aabb, global_transform: &GlobalTransform) -> Aabb3d {
    Aabb3d {
        min: global_transform.transform_point((aabb.center - aabb.half_extents).into()).into(),
        max: global_transform.transform_point((aabb.center + aabb.half_extents).into()).into(),
    }
    
}

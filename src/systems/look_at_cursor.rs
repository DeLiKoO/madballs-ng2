use bevy::prelude::*;
use crate::player::PLAYER_HEIGHT;
use crate::components::{Ground, KeyboardControlled};

pub(crate) fn look_at_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut player_transform_query: Query<&mut Transform, With<KeyboardControlled>>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Single<&Window>,
) {
    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance) + ground.up() * PLAYER_HEIGHT / 2.0;

    for mut transform in &mut player_transform_query {
        transform.look_at(point, ground.up());
    }
}
use bevy::prelude::*;

use crate::components::KeyboardControlled;

const MOVEMENT_SPEED: f32 = 10.0;

pub(crate) fn move_kc_entity(
    mut query: Query<(&KeyboardControlled, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    for (_, mut transform) in &mut query {
        let mut translation: Vec3 = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            translation = translation.mul_add(Vec3::ONE, Vec3::new(0.0, 0.0, -1.0));
        }
        if keys.pressed(KeyCode::KeyA) {
            translation = translation.mul_add(Vec3::ONE, Vec3::new(-1.0, 0.0, 0.0));
        }
        if keys.pressed(KeyCode::KeyS) {
            translation = translation.mul_add(Vec3::ONE, Vec3::new(0.0, 0.0, 1.0));
        }
        if keys.pressed(KeyCode::KeyD) {
            translation = translation.mul_add(Vec3::ONE, Vec3::new(1.0, 0.0, 0.0));
        }
        transform.translation += translation * MOVEMENT_SPEED * timer.delta_secs();
        ()
    }
}

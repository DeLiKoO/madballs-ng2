use bevy::prelude::*;

use crate::components::KeyboardControlled;

pub(crate) fn move_kc_entity(
    mut query: Query<(&KeyboardControlled, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    for (_, mut transform) in &mut query {
        let mut translation: Vec3 = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            translation = (0.0, 0.0, 1.0).into();
        }
        if keys.pressed(KeyCode::KeyA) {
            translation = (-1.0, 0.0, 0.0).into();
        }
        if keys.pressed(KeyCode::KeyS) {
            translation = (0.0, 0.0, -1.0).into();
        }
        if keys.pressed(KeyCode::KeyD) {
            translation = (1.0, 0.0, 0.0).into();
        }
        transform.translation += translation * timer.delta_secs();
        ()
    }
}

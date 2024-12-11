use bevy::prelude::*;

use crate::components::*;

fn apply_damage(
    mut h_query: Query<(&mut Health, &GlobalTransform)>,
    d_query: Query<(&Damage, &GlobalTransform)>
) {
    for (mut health, _) in &mut h_query {
        for (damage, _) in &d_query {
            health.points = health.points - damage.points;
            // TODO: destroy damage
        }
    }
}
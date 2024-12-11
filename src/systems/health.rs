use bevy::prelude::*;

use crate::components::Health;

fn print_health(query: Query<&Health>) {
    for health in &query {
        println!("health: {}", health.points);
    }
}

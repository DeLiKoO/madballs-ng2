use bevy::prelude::*;
use bevy_health_bar3d::prelude::{HealthBarPlugin, Percentage};

use crate::components::Health;

pub struct CustomHealthBarPlugin;

impl Plugin for CustomHealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HealthBarPlugin::<Health>::default());
    }
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.points / self.max
    }
}


//! Plays animations from a skinned glTF.

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

use crate::{health::CustomHealthBarPlugin, weapon::WeaponPlugin};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 14.0,
                    ..default()
                },
                text_color: bevy::color::Color::Srgba(bevy::color::palettes::basic::LIME),
                enabled: true,
                ..default()
            },
        })
        .add_plugins(
            WorldInspectorPlugin::new()
        )
        .add_plugins(PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()))
        .add_plugins(WeaponPlugin)
        .add_plugins(CustomHealthBarPlugin)
        .add_systems(Startup, crate::player::spawn_player_character)
        .add_systems(Startup, crate::world::spawn_grid_plane)
        .add_systems(Startup, crate::world::spawn_camera)
        .add_systems(Startup, crate::world::spawn_lights)
        .add_systems(Update, crate::systems::draw_cursor::draw_cursor)
        .add_systems(Update, crate::systems::keyboard_control::move_kc_entity)
        .add_systems(Update, crate::systems::look_at_cursor::look_at_cursor)
        .run();
}

mod player;
mod weapon;
mod components;
mod systems;
mod world;
mod health;

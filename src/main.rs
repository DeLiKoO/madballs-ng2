//! Plays animations from a skinned glTF.

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::Ground;
use bevy::pbr::{CascadeShadowConfigBuilder, MeshMaterial3d};
use std::f32::consts::PI;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

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
        .add_systems(Startup, crate::player::spawn_player_character)
        .add_systems(Startup, spawn_grid_plane)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, setup_lights)
        .add_systems(Update, crate::systems::draw_cursor::draw_cursor)
        .add_systems(Update, crate::systems::keyboard_control::move_kc_entity)
        .add_systems(Update, crate::systems::look_at_cursor::look_at_cursor)
        .add_systems(Update, crate::systems::shoot_on_click::shoot_on_click)
        .run();
}

/// set up a 3D scene to test shadow biases and perspective projections
fn setup_lights(
    mut commands: Commands,
) {
    // commands.spawn((
    //     PointLight {
    //         intensity: 1_000_000.0,
    //         range: 500.0,
    //         color: Color::WHITE,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(5.0, 5.0, 0.0),
    // ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI / 2., -PI / 4.)),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 25.0,
            ..default()
        }
        .build(),
    ));
}

fn spawn_grid_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(Color::WHITE))),
        Ground,
    ));
}

// Spawns the camera.
fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 30.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}

mod player;
mod components;
mod systems;
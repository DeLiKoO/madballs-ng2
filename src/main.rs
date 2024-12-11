//! Plays animations from a skinned glTF.

use bevy::{prelude::*, pbr::MeshMaterial3d};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use player::spawn_player_character;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(
            WorldInspectorPlugin::new()
        )
        .add_systems(Startup, spawn_player_character)
        .add_systems(Startup, spawn_grid_plane)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, crate::systems::keyboard_control::move_kc_entity)
        .run();
}

fn spawn_grid_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(Color::WHITE))),
    ));
}

// Spawns the camera.
fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}

mod player;
mod components;
mod systems;
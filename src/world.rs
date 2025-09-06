use bevy::pbr::{CascadeShadowConfigBuilder, MeshMaterial3d};
use bevy::prelude::*;
use crate::components::Ground;
use std::f32::consts::PI;

/// set up a 3D scene to test shadow biases and perspective projections
pub(crate) fn spawn_lights(mut commands: Commands) {
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

pub(crate) fn spawn_grid_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10));
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(Color::from(Color::WHITE))),
        Ground,
    ));
}

// Spawns the camera.
pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 30.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

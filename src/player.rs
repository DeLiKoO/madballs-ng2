use bevy::prelude::*;
use crate::components::KeyboardControlled;
use crate::components::Health;

const PLAYER_HEIGHT: f32 = 2.0;

#[derive(Bundle)]
struct PlayerBundle {
    kc: KeyboardControlled,
    // name: String,
    health: Health,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

pub(crate) fn spawn_player_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(
        PlayerBundle {
            kc: KeyboardControlled {},
            health: Health { points: 100.0 },
            mesh: Mesh3d(meshes.add(Sphere::new(PLAYER_HEIGHT / 2.0).mesh())),
            material: MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Srgba::hex("#ffd891").unwrap().into(),
                metallic: 1.0,
                perceptual_roughness: 0.0,
                ..StandardMaterial::default()
            })),
            transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, 0.0),
        }
    );
}
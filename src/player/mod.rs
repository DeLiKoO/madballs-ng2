use bevy::prelude::*;
use bevy_health_bar3d::prelude::{BarSettings, BarOrientation};
use crate::components::KeyboardControlled;
use crate::components::Health;
use crate::components::Player;
use crate::weapon::components::WeaponType;
use crate::weapon::components::WithWeapon;

pub(crate) const PLAYER_HEIGHT: f32 = 2.0;

#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    player: Player,
    // name: String,
    health: Health,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    pub(crate) transform: Transform,
}

pub(crate) fn spawn_player_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(
        (PlayerBundle {
            player: Player {},
            health: Health { points: 100.0, max: 100.0 },
            mesh: Mesh3d(meshes.add(Sphere::new(PLAYER_HEIGHT / 2.0).mesh())),
            material: MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Srgba::hex("#2b49ab").unwrap().into(),
                metallic: 1.0,
                perceptual_roughness: 0.0,
                ..StandardMaterial::default()
            })),
            transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, 0.0),
        },
        KeyboardControlled {},
        BarSettings::<Health> {
            width: 3.0,
            offset: 2.0,
            orientation: BarOrientation::Horizontal, // default is horizontal
            ..default()
        },
        WithWeapon {
            weapon_type: WeaponType::Gun,
        }),
    );

}


pub(crate) fn spawn_dummy_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(
        (PlayerBundle {
            player: Player {},
            health: Health { points: 100.0, max: 100.0 },
            mesh: Mesh3d(meshes.add(Sphere::new(PLAYER_HEIGHT / 2.0).mesh())),
            material: MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Srgba::hex("#ab2b2bff").unwrap().into(),
                metallic: 1.0,
                perceptual_roughness: 0.0,
                ..StandardMaterial::default()
            })),
            transform: Transform::from_xyz(3.0, PLAYER_HEIGHT / 2.0, 3.0),
        },
        BarSettings::<Health> {
            width: 3.0,
            offset: 2.0,
            orientation: BarOrientation::Horizontal, // default is horizontal
            ..default()
        },
        WithWeapon {
            weapon_type: WeaponType::Gun,
        }),
    );

}
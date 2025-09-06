use bevy::prelude::*;
use bevy_health_bar3d::prelude::{BarSettings, BarOrientation};
use crate::components::KeyboardControlled;
use crate::components::Health;
use crate::components::Player;

pub(crate) const PLAYER_HEIGHT: f32 = 2.0;

#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    player: Player,
    kc: KeyboardControlled,
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
    wms: Res<crate::weapon::resources::WeaponAssets>,
) {
    let player_entity_id = commands.spawn(
        (PlayerBundle {
            player: Player {},
            kc: KeyboardControlled {},
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
        BarSettings::<Health> {
            width: 3.0,
            offset: 2.0,
            orientation: BarOrientation::Horizontal, // default is horizontal
            ..default()
        }),
    ).id();

    let weapon_entity_id = commands.spawn(
        // crate::weapon::gun(wms)
        crate::weapon::systems::rocket_launcher(wms)
    ).id();

    // Add child to the parent.
    commands.entity(player_entity_id).add_child(weapon_entity_id);

}
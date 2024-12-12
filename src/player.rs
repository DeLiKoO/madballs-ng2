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

#[derive(Bundle)]
struct WeaponBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

pub(crate) fn spawn_player_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let player_entity_id = commands.spawn(
        PlayerBundle {
            kc: KeyboardControlled {},
            health: Health { points: 100.0 },
            mesh: Mesh3d(meshes.add(Sphere::new(PLAYER_HEIGHT / 2.0).mesh())),
            material: MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Srgba::hex("#2b49ab").unwrap().into(),
                metallic: 1.0,
                perceptual_roughness: 0.0,
                ..StandardMaterial::default()
            })),
            transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, 0.0),
        }
    ).id();
    // Another way is to use the add_child function to add children after the parent
    // entity has already been spawned.
    let weapon_entity_id = commands
        .spawn(
            WeaponBundle {
                // damage: 1.0,
                mesh: Mesh3d(meshes.add(Cuboid::default().mesh())),
                material: MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Srgba::hex("#0d1117").unwrap().into(),
                    metallic: 1.0,
                    perceptual_roughness: 0.0,
                    ..StandardMaterial::default()
                })),
                transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, 0.0).with_scale(Vec3 { x: 0.5, y: 0.5, z: 3.0 }),
            }
        )
        .id();

    // Add child to the parent.
    commands.entity(player_entity_id).add_child(weapon_entity_id);
}
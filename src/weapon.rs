use bevy::prelude::*;
use crate::{components::Weapon, player::PLAYER_HEIGHT};

#[derive(Bundle)]
pub(crate) struct WeaponBundle {
    weapon: Weapon,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

struct WeaponModel {
    mesh: Handle<Mesh>, // NOTICE: Shouldn't we simply store AssetIds ?
    material: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub(crate) struct WeaponModels {
    default: WeaponModel,
}

pub(crate) fn init_weapon_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) { let wms = WeaponModels {
        default: WeaponModel {
            mesh: meshes.add(Cuboid::default().mesh()),
            material: materials.add(StandardMaterial {
                    base_color: Srgba::hex("#0d1117").unwrap().into(),
                    metallic: 1.0,
                    perceptual_roughness: 0.0,
                    ..StandardMaterial::default()
            })
        },
    };
    commands.insert_resource(wms);
}

pub(crate) fn default_weapon(
    wms: Res<WeaponModels>,
) -> WeaponBundle {
    WeaponBundle {
                // damage: 1.0,
                mesh: Mesh3d(wms.default.mesh.clone()),
                material: MeshMaterial3d(wms.default.material.clone()),
                transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, 0.0).with_scale(Vec3 { x: 0.5, y: 0.5, z: 3.0 }),
                weapon: Weapon {  muzzle_pos: Vec3 { x: 0.0, y: 0.0, z: -0.75 } },
            }
}

// pub(crate) fn spawn_weapon(
//     mut commands: Commands,
//     wms: Res<WeaponModels>,
// ) {
//     let weapon_entity_id = commands
//         .spawn(
//             default_weapon(wms)
//         )
//         .id();

//     // Add child to the parent.
//     commands.entity(player_entity_id).add_child(weapon_entity_id);
// }
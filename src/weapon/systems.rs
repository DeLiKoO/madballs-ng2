use avian3d::prelude::*;
use bevy::prelude::*;
use crate::{components::Damage, player::PLAYER_HEIGHT, systems::shoot_on_click::ShootEvent, weapon::{components::{Bullet, Weapon, WeaponBundle}, resources::{WeaponAssets, WeaponMaterials, WeaponMeshes}}};

pub(crate) fn init_weapon_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) { let assets = WeaponAssets {
        meshes: WeaponMeshes {
            cuboid: meshes.add(Cuboid::default().mesh()),
            cylinder: meshes.add(Cylinder::default().mesh()),
        },
        materials: WeaponMaterials {
            default: materials.add(StandardMaterial {
                    base_color: Srgba::hex("#0d1117").unwrap().into(),
                    metallic: 1.0,
                    perceptual_roughness: 0.0,
                    ..StandardMaterial::default()
            }),
            silver_bullet: materials.add(StandardMaterial {
                base_color: Srgba::hex("#2d2d2d").unwrap().into(),
                metallic: 1.0,
                perceptual_roughness: 0.0,
                ..StandardMaterial::default()
            })
        },
    };
    commands.insert_resource(assets);
}

pub(crate) fn rocket_launcher(
    assets: Res<WeaponAssets>,
) -> WeaponBundle {
    WeaponBundle {
                // damage: 1.0,
                mesh: Mesh3d(assets.meshes.cuboid.clone()),
                material: MeshMaterial3d(assets.materials.default.clone()),
                transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, 0.0).with_scale(Vec3 { x: 0.5, y: 0.5, z: 3.0 }),
                weapon: Weapon {  muzzle_pos: Vec3 { x: 0.0, y: 0.0, z: -0.75 } },
            }
}

pub(crate) fn gun(
    assets: Res<WeaponAssets>,
) -> WeaponBundle {
    WeaponBundle {
                // damage: 1.0,
                mesh: Mesh3d(assets.meshes.cuboid.clone()),
                material: MeshMaterial3d(assets.materials.default.clone()),
                transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, -1.0).with_scale(Vec3 { x: 0.25, y: 0.25, z: 0.75 }),
                weapon: Weapon {  muzzle_pos: Vec3 { x: 0.0, y: 0.0, z: -1.75 } },
            }
}


pub(crate) fn weapon_shoot(
    assets: Res<WeaponAssets>,
    mut commands: Commands,
    mut eventreader_shoot: EventReader<ShootEvent>,
) {
    for ev in eventreader_shoot.read() {
        commands.spawn((
            Bullet { },
            Damage { points: 10.0 }, // TODO: impl Clone for Damage
            Mesh3d(assets.meshes.cylinder.clone()),
            MeshMaterial3d(assets.materials.silver_bullet.clone()),
            Transform::from_translation(ev.from)
                .with_scale(Vec3 { x: 0.2, y: 0.2, z: 0.5 })
                .with_rotation(ev.rotation),
            RigidBody::Dynamic,
            Mass(1.0),
            ExternalForce::new(ev.rotation * Vec3::NEG_Z * 1000.0).with_persistence(false),
            GravityScale(0.0),
        ));
    }
}

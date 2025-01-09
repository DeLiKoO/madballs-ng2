use bevy::prelude::*;
use avian3d::prelude::*;
use crate::{components::{Player, Bullet, Damage, KeyboardControlled, Weapon}, player::PLAYER_HEIGHT};

#[derive(Bundle)]
pub(crate) struct BulletBundle {
    bullet: Bullet,
    pub(crate) damage: Damage,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    pub(crate) transform: Transform,
    rigid_body: RigidBody,
    mass: Mass,
    force: ExternalForce,
    gravity_scale: GravityScale,
}

pub(crate) fn shoot_on_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    player_query: Query<(Entity, &Children), (With<Player>, With<KeyboardControlled>)>,
    weapon_query: Query<(Entity, &Parent, &GlobalTransform), With<Weapon>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (player_entity, _) in player_query.iter() {
            for (_, weapon_parent, weapon_global_transform) in weapon_query.iter() {
                if player_entity.eq(&weapon_parent.get()) {
                    let mut force = ExternalForce::new(weapon_global_transform.rotation() * Vec3::NEG_Z * 1000.0).with_persistence(false);
                    commands.spawn(BulletBundle {
                        bullet: Bullet { }, // TODO: Set direction from player transform
                        damage: Damage { points: 10.0 }, // TODO: impl Clone for Damage
                        mesh: Mesh3d(meshes.add(Cylinder::default().mesh())),
                        material: MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Srgba::hex("#2d2d2d").unwrap().into(),
                            metallic: 1.0,
                            perceptual_roughness: 0.0,
                            ..StandardMaterial::default()
                        })),
                        transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0, 0.0)
                            .with_translation(weapon_global_transform.translation())
                            // TODO: Translate so it spawns at end of weapon muzzle 
                            .with_scale(Vec3 { x: 0.2, y: 0.2, z: 0.5 })
                            .with_rotation(weapon_global_transform.rotation()),
                        rigid_body: RigidBody::Dynamic,
                        mass: Mass(1.0),
                        force: force, // TODO: Compute from weapon angle
                        gravity_scale: GravityScale(0.0),
                    });
                }
            }
        }
    }
}
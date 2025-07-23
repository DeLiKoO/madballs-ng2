use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::{Player, Bullet, Damage, KeyboardControlled, Weapon};

pub(crate) fn shoot_on_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    player_query: Query<(Entity, &Children), (With<Player>, With<KeyboardControlled>)>,
    weapon_query: Query<(Entity, &Weapon, &Parent, &GlobalTransform), With<Weapon>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (player_entity, _) in player_query.iter() {
            for (_, weapon, weapon_parent, weapon_global_transform) in weapon_query.iter() {
                if player_entity.eq(&weapon_parent.get()) {
                    commands.spawn((
                        Bullet { },
                        Damage { points: 10.0 }, // TODO: impl Clone for Damage
                        Mesh3d(meshes.add(Cylinder::default().mesh())),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Srgba::hex("#2d2d2d").unwrap().into(),
                            metallic: 1.0,
                            perceptual_roughness: 0.0,
                            ..StandardMaterial::default()
                        })),
                        Transform::from_translation(weapon_global_transform.transform_point(weapon.muzzle_pos))
                            // TODO: Translate so it spawns at end of weapon muzzle 
                            .with_scale(Vec3 { x: 0.2, y: 0.2, z: 0.5 })
                            .with_rotation(weapon_global_transform.rotation()),
                        RigidBody::Dynamic,
                        Mass(1.0),
                        ExternalForce::new(weapon_global_transform.rotation() * Vec3::NEG_Z * 1000.0).with_persistence(false),
                        GravityScale(0.0),
                    ));
                }
            }
        }
    }
}
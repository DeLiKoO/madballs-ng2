use bevy::prelude::*;
use crate::{components::{KeyboardControlled, Player}, weapon::components::Weapon};

#[derive(Event)]
pub(crate) struct ShootEvent {
    pub(crate) player_entity: Entity,
    pub(crate) weapon_entity: Entity,
    pub(crate) from: Vec3,
    pub(crate) rotation: Quat,
}

pub(crate) fn shoot_on_click(
    mut eventwriter_shoot: EventWriter<ShootEvent>,
    buttons: Res<ButtonInput<MouseButton>>,
    player_query: Query<(Entity, &Children), (With<Player>, With<KeyboardControlled>)>,
    weapon_query: Query<(Entity, &Weapon, &Parent, &GlobalTransform), With<Weapon>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (player_entity, _) in player_query.iter() {
            for (weapon_entity, weapon, weapon_parent, weapon_global_transform) in weapon_query.iter() {
                if player_entity.eq(&weapon_parent.get()) {
                    eventwriter_shoot.send(ShootEvent { 
                        player_entity: player_entity,
                        weapon_entity: weapon_entity,
                        from: weapon_global_transform.transform_point(weapon.muzzle_pos),
                        rotation: weapon_global_transform.rotation(),
                    });
                }
            }
        }
    }
}

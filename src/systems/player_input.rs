use bevy::{input::mouse::MouseWheel, prelude::*};
use crate::{components::{KeyboardControlled, Player}, weapon::{components::Weapon, ChangeWeaponDirection, ChangeWeaponEvent, ShootEvent}};

pub(crate) fn shoot_on_click(
    mut evw_shoot: EventWriter<ShootEvent>,
    buttons: Res<ButtonInput<MouseButton>>,
    player_query: Query<(Entity, &Children), (With<Player>, With<KeyboardControlled>)>,
    weapon_query: Query<(Entity, &Weapon, &Parent, &GlobalTransform), With<Weapon>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (player_entity, _) in player_query.iter() {
            for (weapon_entity, weapon, weapon_parent, weapon_global_transform) in weapon_query.iter() {
                if player_entity.eq(&weapon_parent.get()) {
                    evw_shoot.send(ShootEvent { 
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


pub(crate) fn change_weapon_on_mousewheel(
    mut evw_change_weapon: EventWriter<ChangeWeaponEvent>,
    mut evr_scroll: EventReader<MouseWheel>,
    player_query: Query<(Entity, &Children), (With<Player>, With<KeyboardControlled>)>,
    weapon_query: Query<(Entity, &Weapon, &Parent, &GlobalTransform), With<Weapon>>,
) {
    if evr_scroll.is_empty() {
        return;
    }
    for (player_entity, _) in player_query.iter() {
        for (weapon_entity, weapon, weapon_parent, weapon_global_transform) in weapon_query.iter() {
            if player_entity.eq(&weapon_parent.get()) {
            let mut y_sum = 0.0;
            let direction;
            for ev in evr_scroll.read() {
                y_sum += ev.y;
            }
            if y_sum.is_sign_positive() {
                direction = ChangeWeaponDirection::Next;
            }
            else if y_sum.is_sign_negative() {
                direction = ChangeWeaponDirection::Previous;
            } else {
                return; //noop
            }
            evw_change_weapon.send(ChangeWeaponEvent { 
                player_entity: player_entity,
                current_weapon_entity: weapon_entity,
                direction
            });
            }
        }
    }
}

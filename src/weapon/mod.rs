use bevy::prelude::*;

use crate::{components::Player, systems::player_input::ChangeWeaponEvent, weapon::{components::{Weapon, WeaponType, WithWeapon}, resources::WeaponAssets}};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            // TODO: Break the dependency to spawn_player_character by introducing a ChangeWeaponEvent
            //       this event will be triggered once the Player has been spawned 
            // .add_event::<ChangeWeaponEvent>()
            .add_systems(Startup, systems::init_weapon_assets.before(crate::player::spawn_player_character))
            .add_systems(Update, on_with_weapon_component_changed)
            .add_systems(Update, on_change_weapon_event)
            // .add_systems(Update, crate::systems::player_input::shoot_on_click)
            // .add_systems(Update, crate::systems::player_input::change_weapon_on_mousewheel)
            .add_systems(Update, systems::weapon_shoot.after(crate::systems::player_input::shoot_on_click))
            .add_systems(PostUpdate, crate::systems::despawn_out_of_world_bullets::despawn_out_of_world_bullets)
            ;
    }
}

// #[derive(Event)]
// pub(crate) struct ChangeWeaponEvent {
//     pub(crate) weapon_parent_entity: Entity,
//     pub(crate) weapon_type: WeaponType,
// }

fn on_with_weapon_component_changed(
    player_query: Query<
        (Entity, &WithWeapon),
        (With<Player>, Changed<WithWeapon>), 
    >,
    existing_weapon_query: Query<
    (Entity, &Weapon, &Parent),
    >,
    mut commands: Commands,
    assets: Res<WeaponAssets>,
) {
    for (player_entity, with_weapon_component) in player_query.iter() {
        let new_weapon_entity = commands.spawn(
            match with_weapon_component.weapon_type {
                WeaponType::RocketLauncher => crate::weapon::systems::rocket_launcher(&assets),
                WeaponType::Gun => crate::weapon::systems::gun(&assets),
            }
        ).id();

        // Add child to the parent.
        commands.entity(player_entity).add_child(new_weapon_entity);
        // Remove previous weapon from ECS (and as child of player)
        for (existing_weapon_entity, _, weapon_parent) in &existing_weapon_query {
                if weapon_parent.get() == player_entity {
                    commands.entity(player_entity).remove_children(&[existing_weapon_entity]);
                    commands.entity(existing_weapon_entity).despawn();
                }
            }
    }
}

fn on_change_weapon_event(
    mut player_query: Query<
    (Entity, &mut WithWeapon),
    With<Player>, 
    >,
    mut evr_changed_weapon_event: EventReader<ChangeWeaponEvent>,
) {
    for ev in evr_changed_weapon_event.read() {
        for (player_entity, mut ww) in &mut player_query {
            if ev.player_entity == player_entity {
                if ww.weapon_type == WeaponType::RocketLauncher {
                    ww.weapon_type = WeaponType::Gun
                } 
                else {
                    ww.weapon_type = WeaponType::RocketLauncher
                }
            }
        }
    }
}

pub mod resources; // TODO: Make private once above dependecy is broken
pub mod components;
pub mod systems;
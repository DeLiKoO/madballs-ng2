use bevy::prelude::*;

use crate::{components::Player, weapon::{components::{WeaponType, WithWeapon}, resources::WeaponAssets}};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            // TODO: Break the dependency to spawn_player_character by introducing a ChangeWeaponEvent
            //       this event will be triggered once the Player has been spawned 
            .add_event::<ChangeWeaponEvent>()
            .add_systems(Startup, systems::init_weapon_assets.before(crate::player::spawn_player_character))
            .add_systems(Update, on_with_weapon_component_changed)
            .add_systems(Update, on_changed_weapon_event)
            .add_systems(Update, crate::systems::shoot_on_click::shoot_on_click)
            .add_systems(Update, systems::weapon_shoot.after(crate::systems::shoot_on_click::shoot_on_click))
            .add_systems(PostUpdate, crate::systems::despawn_out_of_world_bullets::despawn_out_of_world_bullets)
            ;
    }
}

#[derive(Event)]
pub(crate) struct ChangeWeaponEvent {
    pub(crate) weapon_parent_entity: Entity,
    pub(crate) weapon_type: WeaponType,
}

fn on_with_weapon_component_changed(
    query: Query<
        (Entity, &WithWeapon),
        (With<Player>, Changed<WithWeapon>), 
    >,
    mut ew_changed_weapon_event: EventWriter<ChangeWeaponEvent>
) {
    for (player_entity, ww) in query.iter() {
        ew_changed_weapon_event.send(ChangeWeaponEvent { weapon_parent_entity: player_entity, weapon_type: ww.weapon_type.clone() });
        // match ww.weapon_type {
        //     WeaponType::RocketLauncher => todo!(),
        //     WeaponType::Gun => todo!(),
        // }
    }
}

fn on_changed_weapon_event(
    assets: Res<WeaponAssets>,
    mut commands: Commands,
    mut er_changed_weapon_event: EventReader<ChangeWeaponEvent>,
) {
    for ev in er_changed_weapon_event.read() {
        let weapon_entity_id = commands.spawn(
            match ev.weapon_type {
                WeaponType::RocketLauncher => crate::weapon::systems::rocket_launcher(&assets),
                WeaponType::Gun => crate::weapon::systems::gun(&assets),
            }
        ).id();

        // Add child to the parent.
        commands.entity(ev.weapon_parent_entity).add_child(weapon_entity_id);
    }
}

pub mod resources; // TODO: Make private once above dependecy is broken
pub mod components;
pub mod systems;
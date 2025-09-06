use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            // TODO: Break the dependency to spawn_player_character by introducing a ChangeWeaponEvent
            //       this event will be triggered once the Player has been spawned 
            .add_systems(Startup, crate::weapon::systems::init_weapon_assets.before(crate::player::spawn_player_character))
            .add_systems(Update, crate::systems::shoot_on_click::shoot_on_click)
            .add_systems(Update, crate::weapon::systems::weapon_shoot.after(crate::systems::shoot_on_click::shoot_on_click))
            .add_systems(PostUpdate, crate::systems::despawn_out_of_world_bullets::despawn_out_of_world_bullets);
    }
}

pub mod resources; // TODO: Make private once above dependecy is broken
pub mod components;
pub mod systems;
pub mod damage;
pub mod despawn_out_of_world_bullets;
pub mod keyboard_control;
pub mod draw_cursor;
pub mod look_at_cursor;
pub mod player_input;

use bevy::prelude::*;


/// Print the up-to-date global coordinates of the player
fn debug_globaltransform(
    query: Query<&GlobalTransform>,
) {
    let gxf = query.single();
    debug!("Player at: {:?}", gxf.translation());
}
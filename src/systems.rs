pub mod health;
pub mod damage;
pub mod keyboard_control;
pub mod draw_cursor;

use bevy::prelude::*;


/// Print the up-to-date global coordinates of the player
fn debug_globaltransform(
    query: Query<&GlobalTransform>,
) {
    let gxf = query.single();
    debug!("Player at: {:?}", gxf.translation());
}
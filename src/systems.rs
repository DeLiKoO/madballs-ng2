pub mod health;
pub mod damage;
pub mod keyboard_control;
pub mod draw_cursor;
pub mod look_at_cursor;
pub mod shoot_on_click;

use bevy::prelude::*;


/// Print the up-to-date global coordinates of the player
fn debug_globaltransform(
    query: Query<&GlobalTransform>,
) {
    let gxf = query.single();
    debug!("Player at: {:?}", gxf.translation());
}
use bevy::prelude::*;

#[derive(Reflect, Component)]
pub(crate) struct Health {
    pub(crate) points: f32,
    pub(crate) max: f32,
}

#[derive(Component)]
pub(crate) struct Damage {
    pub(crate) points: f32,
}

#[derive(Component)]
pub(crate) struct Player {
}

#[derive(Component)]
pub(crate) struct KeyboardControlled {
}


#[derive(Component)]
pub(crate) struct Ground;


// NOTICE: use bevy's Transform
// #[derive(Component)]
// pub struct Position {
//     pub(crate) x: f32,
//     pub(crate) y: f32,
//     pub(crate) z: f32,
// }


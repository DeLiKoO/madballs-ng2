use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct Health {
    pub(crate) points: f32,
}

#[derive(Component)]
pub(crate) struct Damage {
    pub(crate) points: f32,
}

#[derive(Component)]
pub(crate) struct KeyboardControlled {
}



// NOTICE: use bevy's Transform
// #[derive(Component)]
// pub struct Position {
//     pub(crate) x: f32,
//     pub(crate) y: f32,
//     pub(crate) z: f32,
// }


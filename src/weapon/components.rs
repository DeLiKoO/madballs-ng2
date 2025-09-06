use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Bullet {
    // direction: Dir3,
    // speed_meter_per_second: f32,
}

#[derive(Component)]
pub(crate) struct Weapon {
    pub(crate) muzzle_pos: Vec3,
}

#[derive(Clone)]
pub(crate) enum WeaponType {
    RocketLauncher,
    Gun,
}

#[derive(Component)]
pub(crate) struct WithWeapon {
    pub(crate) weapon_type: WeaponType,
}

#[derive(Bundle)]
pub(crate) struct WeaponBundle {
    pub(crate) weapon: Weapon,
    pub(crate) mesh: Mesh3d,
    pub(crate) material: MeshMaterial3d<StandardMaterial>,
    pub(crate) transform: Transform,
}
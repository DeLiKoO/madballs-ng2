use bevy::prelude::*;

pub(crate) struct WeaponMeshes {
    pub(crate) cuboid: Handle<Mesh>, // NOTICE: Shouldn't we simply store AssetIds ?
    pub(crate) cylinder: Handle<Mesh>,
}

pub(crate) struct WeaponMaterials {
    pub(crate) default: Handle<StandardMaterial>,
    pub(crate) silver_bullet: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub(crate) struct WeaponAssets {
    pub(crate) meshes: WeaponMeshes,
    pub(crate) materials: WeaponMaterials,
}

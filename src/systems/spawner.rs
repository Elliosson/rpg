use crate::{
    components::*,
    raws::{spawn_named_entity, RawMaster, RAWS},
};
use bevy::prelude::*;

pub fn spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut to_spawn: ResMut<ToSpawn>,
) {
    let raws: &RawMaster = &RAWS.lock().unwrap();

    for (name, x, y) in to_spawn.items.iter() {
        spawn_named_entity(
            &mut commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            (*x, *y),
            name.clone(),
            raws,
        );
    }

    to_spawn.items.clear();
}

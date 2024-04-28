use std::fs;

use bevy::prelude::*;
use serde::Deserialize;

use crate::raws::{spawn_named_entity, RawMaster};

#[derive(Deserialize, Debug)]
struct MapJson {
    map: Vec<(String, f32, f32)>,
}

pub fn load_map(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    raws: &RawMaster,
) {
    let raw_string =
        fs::read_to_string("./map.json").expect("Should have been able to read the file");

    let map_json: MapJson = serde_json::from_str(&raw_string).expect("Unable to parse JSON");

    for (name, x, y) in map_json.map {
        spawn_named_entity(
            commands,
            &asset_server,
            meshes,
            materials,
            (x, y),
            name.clone(),
            raws,
        );
    }
}

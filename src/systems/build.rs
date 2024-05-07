use crate::{
    components::*,
    raws::{spawn_named_entity, RawMaster, RAWS},
};
use bevy::prelude::*;

pub fn build(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut to_be_builded: Query<(Entity, &Buildable, &ActivatedBy, &PropName)>,
    player: Query<(Entity, &Player, &Transform), Without<Buildable>>,
) {
    let (_, _, player_transform) = player.single();
    let raws: &RawMaster = &RAWS.lock().unwrap();
    for (entity, _, _, name) in to_be_builded.iter_mut() {
        spawn_named_entity(
            &mut commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            (
                player_transform.translation.x,
                player_transform.translation.y,
            ),
            name.name.clone(),
            raws,
        );
        commands.entity(entity).despawn();
    }
}

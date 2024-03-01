use crate::{
    components::*,
    utils::{direction, distance},
};
use bevy::prelude::*;

const AGRO_DISTANCE: f32 = 300.;

pub fn monster_find_target(
    mut commands: Commands,
    mut creatures: Query<(Entity, &Monster, &Transform, Option<&mut Target>)>,
    player: Query<(Entity, &Player, &Transform)>,
) {
    let (player_entity, _, player_transform) = player.single();
    for (monster_entity, _, monster_transform, maybe_target) in creatures.iter_mut() {
        if let Some(mut target) = maybe_target {
            if distance(player_transform, monster_transform) >= AGRO_DISTANCE {
                commands.entity(monster_entity).remove::<Target>();
            } else {
                target.direction = direction(monster_transform, player_transform);
            }
        } else {
            if distance(player_transform, monster_transform) < AGRO_DISTANCE {
                commands.entity(monster_entity).insert(Target {
                    entity: player_entity,
                    direction: direction(monster_transform, player_transform),
                });
            }
        }
    }
}

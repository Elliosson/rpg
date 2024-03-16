use crate::components::*;
use bevy::prelude::*;

pub fn level_up(mut player: Query<(Entity, &mut Level, &mut Lifepoint), With<Player>>) {
    let (_, mut level, mut lifepoint) = player.single_mut();

    if level.xp >= 20. {
        level.level += 1;
        level.xp = 0.;
        lifepoint.max += 10.;
    }
}

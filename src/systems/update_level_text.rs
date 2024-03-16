use crate::components::*;
use bevy::prelude::*;

pub fn update_level_text(
    mut query: Query<&mut Text, With<LevelText>>,
    player: Query<(&Level, &Lifepoint), With<Player>>,
) {
    let (level, life) = player.single();
    for mut text in &mut query {
        // Update the value of the second section
        text.sections[0].value = format!(
            "Level: {}  XP: {:.1}, Life: {:.1}/{:.0}",
            level.level, level.xp, life.life, life.max
        );
    }
}

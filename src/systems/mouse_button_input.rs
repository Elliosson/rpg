use crate::components::*;
use bevy::prelude::*;
use bevy::time::Stopwatch;

pub fn mouse_button_input(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut player: Query<Entity, (With<Player>, Without<IsAttacking>)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        if let Ok(player_entity) = player.get_single_mut() {
            commands.entity(player_entity).insert(IsAttacking {
                start_time: Stopwatch::new(),
            });
        }
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}

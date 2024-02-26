use crate::components::*;
use crate::PLAYER_SPEED;
use bevy::prelude::*;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction_y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction_y += 1.0;
    }

    player_transform.translation.x =
        player_transform.translation.x + direction_x * PLAYER_SPEED * time.delta_seconds();
    player_transform.translation.y =
        player_transform.translation.y + direction_y * PLAYER_SPEED * time.delta_seconds();
}
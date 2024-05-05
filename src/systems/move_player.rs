use crate::components::*;
use crate::PLAYER_SPEED;
use bevy::prelude::*;
pub fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &mut Transform), With<Player>>,
    mut next_inventory_state: ResMut<NextState<InventoryUiState>>,
    inventory_state: Res<State<InventoryUiState>>,
    mut action_bar_used: ResMut<ActionBarUsed>,

    time: Res<Time>,
) {
    let (player_entity, mut player_transform) = player.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    //todo rename the system in player input. and do a separated movement system.
    if keyboard_input.just_pressed(KeyCode::KeyI) {
        if *inventory_state.get() == InventoryUiState::Closed {
            next_inventory_state.set(InventoryUiState::Open);
        } else {
            next_inventory_state.set(InventoryUiState::Closed);
        }
    }

    //Action bar
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        action_bar_used.id = Some(1);
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        action_bar_used.id = Some(2);
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        action_bar_used.id = Some(3);
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        action_bar_used.id = Some(4);
    }
    if keyboard_input.just_pressed(KeyCode::Digit5) {
        action_bar_used.id = Some(5);
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        commands.entity(player_entity).insert(WantToPickup {});
    }

    // Movement
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

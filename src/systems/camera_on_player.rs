use crate::components::*;
use bevy::prelude::*;

pub fn camera_on_player(
    query_player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut query_camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = query_player.single();
    for mut camera in query_camera.iter_mut() {
        camera.translation = player_transform.translation;
    }
}

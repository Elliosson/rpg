use crate::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::FRAC_PI_2;

pub fn rotate_player(
    mut query: Query<(&mut Transform, &DeltaAngle), With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let (mut player, delta_angle) = query.single_mut();
    let window = q_windows.single();
    let player_v2 = player.translation.truncate();

    if let Some(cursor_world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let diff = cursor_world_position - player_v2;
        let angle = diff.y.atan2(diff.x) - FRAC_PI_2;
        player.rotation = Quat::from_rotation_z(angle + delta_angle.delta);
    }
}

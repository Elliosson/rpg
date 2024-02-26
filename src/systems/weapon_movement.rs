use crate::components::*;
use bevy::prelude::*;

pub fn weapon_movement(
    player: Query<&Transform, (With<Player>, Without<EquipedWeapon>)>,
    mut weapon: Query<&mut Transform, (With<EquipedWeapon>, Without<Player>)>,
) {
    let mut weapon_transform = weapon.single_mut();
    let player_transform = player.single();

    let player_radius: f32 = 60.;
    let (_, angle) = player_transform.rotation.to_axis_angle();
    let rotation = player_transform.rotation;

    let asin = rotation.z;

    let true_angle = if asin < 0. { angle } else { -angle };

    let left_hand_angle = true_angle - 1.;

    let dx = player_radius * left_hand_angle.sin();
    let dy = player_radius * left_hand_angle.cos();

    weapon_transform.translation.x = player_transform.translation.x + dx;
    weapon_transform.translation.y = player_transform.translation.y + dy;
    weapon_transform.rotation = player_transform.rotation;
}

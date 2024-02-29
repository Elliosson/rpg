use bevy::prelude::Quat;

pub fn correct_angle(rotation: Quat) -> f32 {
    let (_, angle) = rotation.to_axis_angle();
    let rotation = rotation;
    let asin = rotation.z;
    let true_angle = if asin < 0. { angle } else { -angle };
    return true_angle;
}

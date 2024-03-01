use crate::{components::*, utils::correct_angle};
use bevy::prelude::*;
use rand::Rng;

pub fn move_slim(mut slims: Query<(&mut Transform, &Slim, Option<&Target>)>) {
    let mut rng = rand::thread_rng();
    for (mut transform, _slim, maybe_target) in slims.iter_mut() {
        let mut angle = correct_angle(transform.rotation);

        if let Some(target) = maybe_target {
            transform.rotation = Quat::from_rotation_z(target.direction);
        } else {
            // small chance to rotate
            if rng.gen_range(0..100) < 1 {
                angle += rng.gen_range(-2.0..2.0);
                transform.rotation = Quat::from_rotation_z(angle);
            }
        }

        //  move forward.
        transform.translation.x += angle.sin() * 1.;
        transform.translation.y += angle.cos() * 1.;
    }
}

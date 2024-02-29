use crate::{components::*, utils::correct_angle};
use bevy::prelude::*;
use rand::Rng;

pub fn move_slim(mut slims: Query<(&mut Transform, &Slim)>) {
    let mut rng = rand::thread_rng();
    for (mut transform, _slim) in slims.iter_mut() {
        let mut angle = correct_angle(transform.rotation);

        //1. small chance to rotate
        if rng.gen_range(0..100) < 1 {
            angle += rng.gen_range(-2.0..2.0);
            transform.rotation = Quat::from_rotation_z(angle);
        }

        //2.  move forward.

        transform.translation.x += angle.sin() * 1.;
        transform.translation.y += angle.cos() * 1.;
    }
}

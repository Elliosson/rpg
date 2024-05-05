use std::f32::consts::FRAC_PI_2;

use bevy::prelude::{Quat, Transform};
use sepax2d::{Rotate, Shape};

use crate::components::Sepax2dShape;

pub fn correct_angle(rotation: Quat) -> f32 {
    let (_, angle) = rotation.to_axis_angle();
    let rotation = rotation;
    let asin = rotation.z;
    let true_angle = if asin < 0. { angle } else { -angle };
    return true_angle;
}

pub fn get_shape(transform: &Transform, shape: &Sepax2dShape) -> Box<dyn Shape> {
    match shape {
        Sepax2dShape::Circle(radius) => {
            let object = sepax2d::prelude::Circle::new(
                (transform.translation.x, transform.translation.y),
                *radius,
            );
            return Box::new(object);
        }
        Sepax2dShape::Rectangle(width, height) => {
            //rectangle need to be rotated, somehow.
            let mut object = sepax2d::prelude::Parallelogram::rectangle(
                (transform.translation.x, transform.translation.y),
                *width,
                *height,
            );

            let true_angle = correct_angle(transform.rotation);

            object.rotate(-true_angle);
            object.set_position((
                transform.translation.x - width * f32::cos(true_angle)
                    + height * f32::sin(true_angle),
                transform.translation.y + width * f32::sin(true_angle)
                    - height * f32::cos(true_angle),
            ));
            return Box::new(object);
        }
    };
}

pub fn distance(t1: &Transform, t2: &Transform) -> f32 {
    let dx = t2.translation.x - t1.translation.x;
    let dy = t2.translation.y - t1.translation.y;
    return (dx.powf(2.) + dy.powf(2.)).sqrt();
}

pub fn direction(current: &Transform, target: &Transform) -> f32 {
    let diff = target.translation - current.translation;
    let angle = diff.y.atan2(diff.x) - FRAC_PI_2;
    return angle;
}

use crate::{components::*, utils::get_shape};
use bevy::prelude::*;

pub fn collison(
    mut mobiles: Query<
        (&mut Transform, &Sepax2dShape, &Weight),
        (
            With<Collision>,
            With<Sepax2dShape>,
            With<Mobile>,
            Without<Imobile>,
        ),
    >,
    imobiles: Query<
        (&Transform, &Sepax2dShape),
        (
            With<Collision>,
            With<Sepax2dShape>,
            With<Imobile>,
            Without<Mobile>,
        ),
    >,
) {
    //check mobile again imobile, move the mobile
    for (mut mobile_transform, mobile_shape, _) in mobiles.iter_mut() {
        for (imobile_transform, imobile_shape) in imobiles.iter() {
            let mobile_object = get_shape(&mobile_transform, mobile_shape);
            let imobile_object = get_shape(imobile_transform, imobile_shape);

            let (x_delta, y_delta) =
                sepax2d::prelude::sat_collision(&*imobile_object, &*mobile_object);
            //super messy resolve. at the end. juste forbit to move if colliding
            mobile_transform.translation.x += x_delta;
            mobile_transform.translation.y += y_delta;
        }
    }

    let mut mobiles = mobiles.iter_mut().collect::<Vec<_>>();
    mobiles.sort_by_key(|(_, _, weight)| weight.weight);

    for i in 0..mobiles.len() {
        for j in (i + 1)..mobiles.len() {
            let (transform1, shape1, _) = &mobiles[i];
            let (transform2, shape2, _) = &mobiles[j];

            let object1 = get_shape(&transform1, shape1);
            let object2 = get_shape(&transform2, shape2);

            let (x_delta, y_delta) = sepax2d::prelude::sat_collision(&*object2, &*object1);
            mobiles[i].0.translation.x += x_delta;
            mobiles[i].0.translation.y += y_delta;
        }
    }
}

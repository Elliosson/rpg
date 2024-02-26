use crate::components::*;
use bevy::prelude::*;

pub fn collison(
    mut player: Query<(&mut Transform, &Sepax2dShape), (With<Player>, With<Sepax2dShape>)>,
    collidables: Query<
        (&Transform, &Sepax2dShape),
        (With<Collision>, With<Sepax2dShape>, Without<Player>),
    >,
) {
    let (mut player_transform, shape) = player.single_mut();
    let radius = if let Sepax2dShape::Circle(radius) = shape {
        *radius
    } else {
        println!("Player shape is expected to be a circle!");
        0.
    };

    let character_circle = sepax2d::prelude::Circle::new(
        (
            player_transform.translation.x,
            player_transform.translation.y,
        ),
        radius,
    );

    for (transform, shape) in collidables.iter() {
        let (x_delta, y_delta): (f32, f32) = match shape {
            Sepax2dShape::Circle(radius) => {
                let object = sepax2d::prelude::Circle::new(
                    (transform.translation.x, transform.translation.y),
                    *radius,
                );
                sepax2d::prelude::sat_collision(&object, &character_circle)
            }
            Sepax2dShape::Rectangle(width, height) => {
                let object = sepax2d::prelude::Parallelogram::rectangle(
                    (transform.translation.x, transform.translation.y),
                    *width,
                    *height,
                );
                sepax2d::prelude::sat_collision(&object, &character_circle)
            }
        };
        //super messy resolve. at the end. juste forbit to move if colliding
        player_transform.translation.x += x_delta;
        player_transform.translation.y += y_delta;
    }
}

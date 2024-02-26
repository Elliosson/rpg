use crate::components::*;
use bevy::prelude::*;

pub fn weapon_hit(
    mut player: Query<
        (&mut Transform, &Sepax2dShape),
        (
            With<Player>,
            With<Sepax2dShape>,
            With<IsAttacking>,
            Without<EquipedWeapon>,
        ),
    >,
    collidables: Query<
        (&Transform, &Sepax2dShape),
        (
            With<Collision>,
            With<Sepax2dShape>,
            Without<Player>,
            Without<EquipedWeapon>,
        ),
    >,
    equipped_weapon: Query<
        (&Transform, &Sepax2dShape),
        (With<EquipedWeapon>, With<Sepax2dShape>, Without<Player>),
    >,
) {
    //if the player is attacking
    if let Ok(_) = player.get_single_mut() {
        let (weapon_transform, weapon_shape) = equipped_weapon.single();

        let (width, height) = if let Sepax2dShape::Rectangle(width, height) = weapon_shape {
            (*width, *height)
        } else {
            println!("Weapon shape is expected to be a rectangle!");
            (0., 0.)
        };

        let weapon_rectangle = sepax2d::prelude::Parallelogram::rectangle(
            (
                weapon_transform.translation.x,
                weapon_transform.translation.y,
            ),
            width,
            height,
        );

        for (transform, shape) in collidables.iter() {
            let (x_delta, y_delta): (f32, f32) = match shape {
                Sepax2dShape::Circle(radius) => {
                    let object = sepax2d::prelude::Circle::new(
                        (transform.translation.x, transform.translation.y),
                        *radius,
                    );
                    sepax2d::prelude::sat_collision(&object, &weapon_rectangle)
                }
                Sepax2dShape::Rectangle(width, height) => {
                    let object = sepax2d::prelude::Parallelogram::rectangle(
                        (transform.translation.x, transform.translation.y),
                        *width,
                        *height,
                    );
                    sepax2d::prelude::sat_collision(&object, &weapon_rectangle)
                }
            };
            if x_delta != 0. || y_delta != 0. {
                //dommage or something
                println!("dammage!");
            }
        }
    }
}

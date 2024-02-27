use crate::components::*;
use bevy::prelude::*;

const IS_HIT_MOUVEMENT: f32 = 3.0;

pub fn is_hit_animation(
    mut commands: Commands,
    mut is_hits: Query<(Entity, &mut IsHitAnimation, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut is_hit, mut transform) in is_hits.iter_mut() {
        is_hit.start_time.tick(time.delta()).elapsed_secs();
        let delta_time = is_hit.start_time.elapsed_secs();

        let tangent = (is_hit.dx * is_hit.dx + is_hit.dy * is_hit.dy).sqrt();
        if tangent > 0. {
            let factor = 1. / tangent;
            let delta_x = is_hit.dx * factor * IS_HIT_MOUVEMENT;
            let delta_y = is_hit.dy * factor * IS_HIT_MOUVEMENT;

            //todo be sure to only do it to non-movable object
            //also be sure tha the is hit is not overwrited!
            //and do not move the collision!
            if !is_hit.already_moved {
                transform.translation.x -= delta_x;
                transform.translation.y -= delta_y;
                is_hit.already_moved = true;
            } else if delta_time > 0.1 {
                transform.translation.x += delta_x;
                transform.translation.y += delta_y;
                commands.entity(entity).remove::<IsHitAnimation>();
            }
        } else {
            println!("Error division by 0!");
            commands.entity(entity).remove::<IsHitAnimation>();
        }
    }
}

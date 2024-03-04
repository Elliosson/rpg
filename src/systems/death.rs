use crate::components::*;
use bevy::{prelude::*, utils::HashMap};

pub fn death(
    mut commands: Commands,
    mut creatures: Query<(Entity, &Lifepoint, Option<&LastHitBy>, Option<&mut Level>)>,
) {
    let mut entity_to_xp: HashMap<Entity, f32> = HashMap::new();
    for (entity, lifepoint, maybe_last_hit_by, _) in creatures.iter() {
        if lifepoint.life <= 0. {
            commands.entity(entity).despawn();
            //need to despawn the life bar also(and all the related entity)
            //done in there respective systems

            //give a fixed amount of xp to the killer. do it here right now.
            if let Some(last_hit_by) = maybe_last_hit_by {
                let value = entity_to_xp.entry(last_hit_by.entity).or_default();
                *value += 10.;
            }
        }
    }

    for (key, val) in entity_to_xp.iter() {
        let (_, _, _, maybe_level) = creatures.get_mut(*key).unwrap();
        if let Some(mut level) = maybe_level {
            level.xp += val;
        }
    }
}

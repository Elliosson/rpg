use crate::components::*;
use bevy::prelude::*;

pub fn death(mut commands: Commands, creatures: Query<(Entity, &Lifepoint)>) {
    for (entity, lifepoint) in creatures.iter() {
        if lifepoint.life <= 0. {
            commands.entity(entity).despawn();
            //need to despawn the life bar also(and all the related entity)
            //done in there respective systems
        }
    }
}

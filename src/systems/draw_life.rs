use std::collections::HashMap;

use crate::components::*;
use bevy::{prelude::*, sprite::Mesh2dHandle};

pub fn draw_life(
    mut commands: Commands,
    mut life_bars: Query<(Entity, &mut Transform, &LifeBar), Without<Lifepoint>>,
    creatures: Query<(Entity, &Transform, &Lifepoint), Without<LifeBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut entity_to_lifepoint: HashMap<Entity, (Transform, f32)> = HashMap::new();
    for (entity, transform, lifepoint) in creatures.iter() {
        entity_to_lifepoint.insert(entity, (transform.clone(), lifepoint.life));
    }
    for (entity, mut transform, lifebar) in life_bars.iter_mut() {
        if let Some((creature_transform, life)) = entity_to_lifepoint.get(&lifebar.linked_entity) {
            transform.translation = creature_transform.translation.clone();
            transform.translation.y += 60.;

            //change the size or something.
            commands
                .entity(entity)
                .insert(Mesh2dHandle(meshes.add(Rectangle::new(*life, 1.0))));

        //if not found, remove the lifebar
        } else {
            commands.entity(entity).despawn();
        }
    }
}

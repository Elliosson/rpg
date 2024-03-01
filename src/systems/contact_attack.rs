use crate::{components::*, utils::get_shape};
use bevy::{prelude::*, utils::HashMap};
use sepax2d::Shape;

pub fn contact_attack(
    attacking: Query<(Entity, &ContactAttack, &Target, &Transform, &Sepax2dShape)>,
    mut creatures: Query<(Entity, &Transform, &Sepax2dShape, &mut Lifepoint)>,
) {
    let mut attacked_by_shape: HashMap<Entity, Box<dyn Shape>> = HashMap::new();
    for (_, _, target, transform, sepax_shape) in attacking.iter() {
        let shape = get_shape(transform, sepax_shape);
        attacked_by_shape.insert(target.entity, shape);
    }

    for (attacked_entity, attacking_shape) in attacked_by_shape.iter() {
        if let Ok((_, transform, sepax_shape, mut attacked_life)) =
            creatures.get_mut(*attacked_entity)
        {
            let attacked_shape = get_shape(transform, sepax_shape);
            let (x_delta, y_delta) =
                sepax2d::prelude::sat_collision(&*attacked_shape, &**attacking_shape);
            if x_delta != 0. || y_delta != 0. {
                attacked_life.life -= 1.;
            }
        }
    }
}

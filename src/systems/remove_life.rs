use crate::components::*;
use bevy::prelude::*;

pub fn remove_life(
    mut commands: Commands,
    mut creatures_hitted: Query<(Entity, &mut Lifepoint, &IsHit)>,
) {
    for (entity, mut lifepoint, _is_hit) in creatures_hitted.iter_mut() {
        lifepoint.life = f32::max(0., lifepoint.life - 5.);
        commands.entity(entity).remove::<IsHit>();
    }

    //todo find a way to clear all the is hit component at once.
}

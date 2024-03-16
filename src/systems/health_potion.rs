use crate::components::*;
use bevy::prelude::*;

pub fn health_potion(
    mut commands: Commands,
    potions: Query<(Entity, &HealthPotion, &ActivatedBy)>,
    mut creatures: Query<(Entity, &mut Lifepoint), Without<HealthPotion>>,
) {
    for (entity, potion, activated_by) in potions.iter() {
        if let Ok((_, mut lifepoint)) = creatures.get_mut(activated_by.entity) {
            lifepoint.life += potion.heal;
            lifepoint.life = f32::min(lifepoint.life + potion.heal, lifepoint.max);
        }
        commands.entity(entity).despawn();
    }
}

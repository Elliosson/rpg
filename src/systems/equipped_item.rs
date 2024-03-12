use crate::components::*;
use bevy::prelude::*;

pub fn equipped_item(
    mut commands: Commands,
    mut to_equip: Query<
        (Entity, &mut Visibility),
        (With<UniqueItem>, With<EquipedWeapon>, Without<Transform>),
    >,
    mut to_deequip: Query<
        (Entity, &mut Visibility),
        (With<UniqueItem>, With<Transform>, Without<EquipedWeapon>),
    >,
) {
    for (entity, mut visibility) in to_equip.iter_mut() {
        *visibility = Visibility::Visible;
        commands.entity(entity).insert(Transform::default());
    }

    for (entity, mut visibility) in to_deequip.iter_mut() {
        *visibility = Visibility::Hidden;
        commands.entity(entity).remove::<Transform>();
    }
}

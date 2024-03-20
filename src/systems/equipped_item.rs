use crate::components::*;
use bevy::prelude::*;

pub fn equipped_item(
    mut commands: Commands,
    mut to_equip: Query<
        (Entity, &mut Visibility, Option<&ZLayer>),
        (With<UniqueItem>, With<EquipedWeapon>, Without<Transform>),
    >,
    mut to_deequip: Query<
        (Entity, &mut Visibility),
        (With<UniqueItem>, With<Transform>, Without<EquipedWeapon>),
    >,
) {
    for (entity, mut visibility, maybe_z_layer) in to_equip.iter_mut() {
        *visibility = Visibility::Visible;
        let z = if let Some(z_layer) = maybe_z_layer {
            z_layer.value()
        } else {
            0.
        };
        let mut transform = Transform::default();
        transform.translation.z = z;
        commands.entity(entity).insert(transform);
    }

    for (entity, mut visibility) in to_deequip.iter_mut() {
        *visibility = Visibility::Hidden;
        commands.entity(entity).remove::<Transform>();
    }
}

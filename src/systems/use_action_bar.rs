use crate::components::*;
use bevy::prelude::*;

pub fn use_action_bar(
    mut commands: Commands,
    mut equiped_weapons: Query<
        (Entity, &mut Handle<Image>),
        (With<EquipedWeapon>, Without<SlotButton>),
    >,
    inventory: Res<Inventory>,
    mut action_bar_used: ResMut<ActionBarUsed>,
) {
    if let Some(id) = action_bar_used.id {
        if let Some(inv_case) = inventory.slots.get(&id) {
            match inv_case {
                InventoryCase::Unique(_, to_equip_entity) => {
                    let (equipped_entity, _) = equiped_weapons.single_mut();
                    commands.entity(equipped_entity).remove::<EquipedWeapon>();
                    commands.entity(*to_equip_entity).insert(EquipedWeapon {});
                }
                _ => {}
            }
        }
    }

    action_bar_used.id = None;
}

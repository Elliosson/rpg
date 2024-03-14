use crate::components::*;
use bevy::prelude::*;

pub fn item_slot_button(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &mut UiImage, &SlotButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut equiped_weapons: Query<
        (Entity, &mut Handle<Image>),
        (With<EquipedWeapon>, Without<SlotButton>),
    >,
    inventory: Res<Inventory>,
) {
    for (interaction, mut border_color, mut _ui_image, slot_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(inv_case) = inventory.slots.get(&slot_button.id) {
                    match inv_case {
                        InventoryCase::Unique(_, to_equip_entity) => {
                            let (equipped_entity, _) = equiped_weapons.single_mut();
                            commands.entity(equipped_entity).remove::<EquipedWeapon>();
                            commands.entity(*to_equip_entity).insert(EquipedWeapon {});
                        }
                        _ => {}
                    }
                }

                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}

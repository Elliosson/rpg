use crate::components::*;
use bevy::prelude::*;

pub fn drag_and_drop(
    hovered: Res<ButtonHovered>,
    mut just_released: ResMut<ButtonJustReleased>,
    button_query: Query<(Entity, &InventorySlot, &mut UiImage)>,
    mut inventory: ResMut<Inventory>,
) {
    if let Some(released_entity) = just_released.entity {
        if let Some(hovered_entity) = hovered.entity {
            let hovered_id: i32 = button_query.get(hovered_entity).unwrap().1.id;
            let released_id: i32 = button_query.get(released_entity).unwrap().1.id;

            //special case for the gear
            //todo dirty, to change once I can handle complex object in the inventory.
            //helmet
            if hovered_id == 101 {
                if let Some(slot) = inventory.slots.get(&released_id) {
                    let name = match slot {
                        InventoryCase::Stack(name, _) => name,
                        InventoryCase::Unique(name, _) => name,
                    };
                    if name != "helmet" {
                        return;
                    }
                } else {
                    return;
                }
            }
            //armor
            if hovered_id == 102 {
                if let Some(slot) = inventory.slots.get(&released_id) {
                    let name = match slot {
                        InventoryCase::Stack(name, _) => name,
                        InventoryCase::Unique(name, _) => name,
                    };
                    if name != "armor" {
                        return;
                    }
                } else {
                    return;
                }
            }
            //boots
            if hovered_id == 103 {
                if let Some(slot) = inventory.slots.get(&released_id) {
                    let name = match slot {
                        InventoryCase::Stack(name, _) => name,
                        InventoryCase::Unique(name, _) => name,
                    };
                    if name != "boots" {
                        return;
                    }
                } else {
                    return;
                }
            }

            let hovered_case = inventory.slots.remove(&hovered_id);
            let released_case = inventory.slots.remove(&released_id);

            if let Some(case) = released_case {
                inventory.slots.insert(hovered_id, case);
            }

            if let Some(case) = hovered_case {
                inventory.slots.insert(released_id, case);
            }
        }
    }

    just_released.entity = None;
}

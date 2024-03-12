use crate::components::*;
use bevy::{prelude::*, utils::HashMap};

pub fn drag_and_drop(
    mut commands: Commands,
    mut hovered: ResMut<ButtonHovered>,
    mut just_released: ResMut<ButtonJustReleased>,
    mut button_query: Query<(Entity, &mut InventorySlot, &mut UiImage)>,
    mut inventory: ResMut<Inventory>,
) {
    if let Some(released_entity) = just_released.entity {
        if let Some(hovered_entity) = hovered.entity {
            let hovered_id: i32 = button_query.get(hovered_entity).unwrap().1.id;
            let released_id: i32 = button_query.get(released_entity).unwrap().1.id;

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

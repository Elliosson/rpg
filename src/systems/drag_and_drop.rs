use crate::components::*;
use bevy::{prelude::*, utils::HashMap};

pub fn drag_and_drop(
    mut commands: Commands,
    mut hovered: ResMut<ButtonHovered>,
    mut just_released: ResMut<ButtonJustReleased>,
    mut button_query: Query<(Entity, &mut InventorySlot)>,
) {
    if let Some(released_entity) = just_released.entity {
        if let Some(hovered_entity) = hovered.entity {
            let hovered_id: i32 = button_query.get(hovered_entity).unwrap().1.id;
            let released_id: i32 = button_query.get(released_entity).unwrap().1.id;

            button_query.get_mut(hovered_entity).unwrap().1.id = released_id;
            button_query.get_mut(released_entity).unwrap().1.id = hovered_id;
        }
    }

    just_released.entity = None;
}

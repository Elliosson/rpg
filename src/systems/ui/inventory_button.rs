use crate::components::*;
use bevy::prelude::*;

pub fn inventory_button(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BorderColor,
            &mut UiImage,
            &mut InventorySlot,
        ),
        (Changed<Interaction>, With<Button>),
    >,

    mut pressed: ResMut<ButtonPressed>,
    mut hovered: ResMut<ButtonHovered>,
    mut just_released: ResMut<ButtonJustReleased>,
) {
    for (entity, interaction, mut border_color, mut _ui_image, mut inventory_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                pressed.entity = Some(entity);
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                hovered.entity = Some(entity);

                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
                if pressed.entity == Some(entity) {
                    pressed.entity = None;
                }
                if hovered.entity == Some(entity) {
                    hovered.entity = None;
                }

                //specifically this mean the button was relaseing while not hovering the button.
                if inventory_button.previous_interaction == Interaction::Pressed {
                    just_released.entity = Some(entity);
                }
            }
        }
        inventory_button.previous_interaction = *interaction;
    }
}

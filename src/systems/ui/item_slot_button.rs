use crate::components::*;
use bevy::prelude::*;

pub fn item_slot_button(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &mut UiImage, &SlotButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut action_bar_used: ResMut<ActionBarUsed>,
) {
    for (interaction, mut border_color, mut _ui_image, slot_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                action_bar_used.id = Some(slot_button.id);
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

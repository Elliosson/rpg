use crate::components::*;
use bevy::prelude::*;

pub fn inventory_button(
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &mut UiImage, &InventorySlot),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut border_color, mut _ui_image, inentory_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
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

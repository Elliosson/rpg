use crate::components::*;
use bevy::prelude::*;

pub fn item_slot_button(
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &mut UiImage, &SlotButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    let hammer: Handle<Image> = asset_server.load("hammer_icon.png");
    let sword: Handle<Image> = asset_server.load("sword_icon.png");

    for (interaction, mut border_color, mut ui_image, slot_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if slot_button.id == 1 {
                    *ui_image = UiImage::new(hammer.clone());
                }
                if slot_button.id == 2 {
                    *ui_image = UiImage::new(sword.clone());
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

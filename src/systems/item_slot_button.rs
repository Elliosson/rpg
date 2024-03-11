use crate::components::*;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn item_slot_button(
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut UiImage,
            &Children,
            &SlotButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    let hammer: Handle<Image> = asset_server.load("hammer.png");
    let sword: Handle<Image> = asset_server.load("sword.png");

    for (interaction, mut color, mut border_color, mut ui_image, children, slot_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                // if slot_button.id == 1 {
                //     *ui_image = UiImage::new(hammer.clone());
                // }
                // if slot_button.id == 2 {
                //     *ui_image = UiImage::new(sword.clone());
                // }
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

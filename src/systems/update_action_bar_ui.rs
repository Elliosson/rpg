use crate::components::*;
use bevy::{prelude::*, utils::HashMap};

pub fn update_action_bar_ui(
    mut commands: Commands,
    mut buttons: Query<
        (
            Entity,
            &Interaction,
            &mut BorderColor,
            &mut UiImage,
            &mut SlotButton,
        ),
        With<Button>,
    >,
    inventoy: Res<Inventory>,
    asset_server: Res<AssetServer>,
) {
    for (entity, interaction, mut border_color, mut ui_image, mut slot) in &mut buttons {
        if let Some(slot) = inventoy.slots.get(&slot.id) {
            let name = match slot {
                InventoryCase::Stack(name, _) => name,
                InventoryCase::Unique(name, _) => name,
            };

            //todo, this is not working very well, if for some reason, the image is dropped, i can't reload it.
            //it work only because the image stay loaded in the skill slots.
            //I guess I need to keep the handle stored somewehre.
            let texture_handle: Handle<Image> =
                asset_server.load(format!("{}_icon.png", name.clone()));

            *ui_image = UiImage::new(texture_handle.clone());
        } else {
            let texture_handle: Handle<Image> = asset_server.load("slot.png");
            *ui_image = UiImage::new(texture_handle.clone());
        }
    }
}

use crate::components::*;
use bevy::{prelude::*, utils::HashMap};

pub fn update_inventoty_ui(
    mut commands: Commands,
    mut buttons: Query<
        (
            Entity,
            &Interaction,
            &Children,
            &mut BorderColor,
            &mut UiImage,
            &mut InventorySlot,
        ),
        With<Button>,
    >,
    inventoy: Res<Inventory>,
    asset_server: Res<AssetServer>,
) {
    for (entity, interaction, children, mut border_color, mut ui_image, mut inventory_button) in
        &mut buttons
    {
        if let Some(inv_case) = inventoy.slots.get(&inventory_button.id) {
            let name = match inv_case {
                InventoryCase::Stack(name, _) => name,
                InventoryCase::Unique(name, _) => name,
            };
            let texture_handle: Handle<Image> =
                asset_server.load(format!("{}_icon.png", name.clone()));

            *ui_image = UiImage::new(texture_handle.clone());
        }
    }
}

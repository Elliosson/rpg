use crate::components::*;
use bevy::prelude::*;

pub fn update_action_bar_ui(
    mut buttons: Query<(&mut UiImage, &SlotButton), With<Button>>,
    inventoy: Res<Inventory>,
    asset_server: Res<AssetServer>,
) {
    for (mut ui_image, slot) in &mut buttons {
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

use crate::components::*;
use bevy::prelude::*;

pub fn update_inventoty_ui(
    mut buttons: Query<(&mut UiImage, &Children, &InventorySlot), With<Button>>,
    inventoy: Res<Inventory>,
    asset_server: Res<AssetServer>,
    mut text_query: Query<&mut Text>,
) {
    for (mut ui_image, children, inventory_button) in &mut buttons {
        if let Some(inv_case) = inventoy.slots.get(&inventory_button.id) {
            let name = match inv_case {
                InventoryCase::Stack(name, amount) => {
                    let mut text = text_query.get_mut(children[0]).unwrap();
                    text.sections[0].value = amount.to_string();
                    name
                }
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

use crate::components::*;
use bevy::prelude::*;

pub fn inventory_button(
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &Children,
            &mut BorderColor,
            &mut UiImage,
            &mut InventorySlot,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut pressed: ResMut<ButtonPressed>,
    mut hovered: ResMut<ButtonHovered>,
    mut just_released: ResMut<ButtonJustReleased>,
) {
    let mut released_button: Option<(Entity, i32)> = None;
    let mut hovered_button: Option<(Entity, i32)> = None;
    for (entity, interaction, children, mut border_color, mut _ui_image, mut inventory_button) in
        &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        text.sections[0].value = inventory_button.id.to_string();

        match *interaction {
            Interaction::Pressed => {
                pressed.entity = Some(entity);
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                println!("hovered");
                hovered.entity = Some(entity);

                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
                println!("none");
                println!("{:?}", inventory_button.previous_interaction);
                if pressed.entity == Some(entity) {
                    pressed.entity = None;
                }
                if hovered.entity == Some(entity) {
                    hovered.entity = None;
                }

                //specifically this mean the button was relaseing while not hovering the button.
                if inventory_button.previous_interaction == Interaction::Pressed {
                    println!("was relased");
                    just_released.entity = Some(entity);
                }
            }
        }
        inventory_button.previous_interaction = *interaction;
    }
}

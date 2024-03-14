use crate::components::*;
use bevy::prelude::*;

const INVENTORY_WIDTH: i32 = 5;
const INVENTORY_HEIGHT: i32 = 5;
const CELL_SIZE: f32 = 64.;

pub fn inventory_slot(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, id: i32) {
    let texture_handle = asset_server.load("slot.png");

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(1.0)),
                    width: Val::Px(CELL_SIZE),
                    height: Val::Px(CELL_SIZE),
                    border: UiRect::all(Val::Px(1.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: UiImage::new(texture_handle.clone()),
                border_color: BorderColor(Color::BLACK),
                ..default()
            },
            InventorySlot {
                id,
                previous_interaction: Interaction::None,
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("{}", id),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

pub fn inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(400.0),
                    margin: UiRect::all(Val::Px(2.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,

                    ..default()
                },
                background_color: BackgroundColor(Color::DARK_GRAY),
                ..default()
            },
            InventoryScreen,
        ))
        .with_children(|parent| {
            for y in 0..INVENTORY_HEIGHT {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(1.0)),
                            width: Val::Percent(100.0),
                            height: Val::Px(CELL_SIZE),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        for x in 0..INVENTORY_WIDTH {
                            inventory_slot(parent, &asset_server, y * INVENTORY_WIDTH + x);
                        }
                    });
            }
        });
}

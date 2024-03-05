use crate::components::*;
use bevy::{prelude::*, utils::HashMap};

pub fn inventory_slot(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, id: i32) {
    let texture_handle = asset_server.load("slot.png");

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(1.0)),
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
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
            InventorySlot { id },
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

pub fn inventory_ui(
    mut commands: Commands,
    inventory_ui: Res<InventoryUi>,
    asset_server: Res<AssetServer>,
) {
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
            for y in 0..10 {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(2.0)),
                            width: Val::Percent(100.0),
                            height: Val::Px(32.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        for x in 0..10 {
                            inventory_slot(parent, &asset_server, y * 10 + x);
                        }
                    });
            }
        });
}

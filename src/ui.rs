pub use crate::components::*;
use bevy::prelude::*;

pub fn slot_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, id: i32) {
    let texture_handle: Handle<Image> = if id == 1 {
        asset_server.load("hammer_icon.png")
    } else if id == 2 {
        asset_server.load("sword_icon.png")
    } else {
        asset_server.load("mana_potion_icon.png")
    };

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(65.0),
                height: Val::Px(85.0),
                margin: UiRect::all(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            //this will remove the color of the image if in front of an image
            parent.spawn(TextBundle::from_section(
                format!("{}", id),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::rgba(0.9, 0.9, 0.9, 0.9),
                },
            ));
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(4.0)),
                        width: Val::Px(65.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage::new(texture_handle.clone()),
                    border_color: BorderColor(Color::BLACK),
                    // background_color: BackgroundColor(Color::WHITE),
                    ..default()
                },
                SlotButton { id },
            ));
        });
}

pub fn spawn_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                // Create a TextBundle that has a Text with a single section.
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "",
                    TextStyle {
                        // This font is loaded and will be used instead of the default font.
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                ) // Set the justification of the Text
                .with_text_justify(JustifyText::Center)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                }),
                LevelText {},
            ));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(85.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for i in 1..6 {
                        slot_button(parent, asset_server, i);
                    }
                });
        });
}

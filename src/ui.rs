pub use crate::components::*;
use bevy::prelude::*;

pub fn spawn_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle = asset_server.load("slot.png");

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
                        height: Val::Px(65.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for i in 1..6 {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        margin: UiRect::all(Val::Px(4.0)),
                                        width: Val::Px(65.0),
                                        height: Val::Px(65.0),
                                        border: UiRect::all(Val::Px(5.0)),
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
                                SlotButton { id: i },
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    format!("{}", i),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ));
                            });
                    }
                });
        });
}

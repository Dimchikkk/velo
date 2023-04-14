use bevy::prelude::*;
use bevy_ui_borders::BorderColor;



use super::ui_helpers::{get_tooltip, TextManipulation, TextManipulationAction, Tooltip};

pub fn add_text_manipulation(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text_manipulation: TextManipulationAction,
    font: Handle<Font>,
) -> Entity {
    let (image, text) = match text_manipulation.action_type {
        TextManipulation::Cut => (asset_server.load("cut-text.png"), "Cut text"),
        TextManipulation::Paste => (
            asset_server.load("paste-text.png"),
            "Paste text from clipboard",
        ),
        TextManipulation::Copy => (asset_server.load("copy-text.png"), "Copy text to clipboard"),
        TextManipulation::OpenAllLinks => (
            asset_server.load("open-all-links.png"),
            "Open all links in text",
        ),
    };
    commands
        .spawn((
            ButtonBundle {
                background_color: Color::Rgba {
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                    alpha: 0.5,
                }
                .into(),
                image: image.into(),
                style: Style {
                    size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        top: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            text_manipulation,
        ))
        .with_children(|builder| {
            builder.spawn((get_tooltip(font, text.to_string(), 14.), Tooltip));
        })
        .id()
}

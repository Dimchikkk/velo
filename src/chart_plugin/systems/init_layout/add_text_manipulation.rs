use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::chart_plugin::ui_helpers::GenericButton;

use super::ui_helpers::{get_tooltip, TextManipulation, TextManipulationAction, Tooltip};

pub fn add_text_manipulation(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text_manipulation: TextManipulationAction,
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
    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(5.)),
                size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                ..default()
            },
            background_color: Color::BLACK.with_a(0.5).into(),
            ..default()
        })
        .id();
    let button = commands
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
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(-2.),
                        right: Val::Px(0.),
                        top: Val::Px(-2.),
                        bottom: Val::Px(0.),
                    },
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            text_manipulation,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((get_tooltip(text.to_string(), 14.), Tooltip));
        })
        .id();
    commands.entity(top).add_child(button);
    top
}

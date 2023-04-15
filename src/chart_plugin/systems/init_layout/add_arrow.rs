use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::chart_plugin::ui_helpers::GenericButton;

use super::ui_helpers::{get_tooltip, ArrowMode, ArrowType, Tooltip};

pub fn add_arrow(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    arrow_mode: ArrowMode,
    font: Handle<Font>,
) -> Entity {
    let (image, text) = match arrow_mode.arrow_type {
        ArrowType::Line => (asset_server.load("line.png"), "Enable line mode"),
        ArrowType::Arrow => (asset_server.load("arrow.png"), "Enable single arrow mode"),
        ArrowType::DoubleArrow => (
            asset_server.load("double-arrow.png"),
            "Enable double arrow mode",
        ),
        ArrowType::ParallelLine => (
            asset_server.load("parallel-line.png"),
            "Enable parallel line mode",
        ),
        ArrowType::ParallelArrow => (
            asset_server.load("parallel-arrow.png"),
            "Enable parallel arrow mode",
        ),
        ArrowType::ParallelDoubleArrow => (
            asset_server.load("parallel-double-arrow.png"),
            "Enable parallel double arrow mode",
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
                    size: Size::new(Val::Percent(12.), Val::Percent(100.)),
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
            arrow_mode,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((get_tooltip(font, text.to_string(), 14.), Tooltip));
        })
        .id()
}

use bevy::prelude::*;

use crate::{
    themes::Theme,
    ui_plugin::ui_helpers::{GenericButton, TooltipPosition},
};

use super::ui_helpers::{get_tooltip, Tooltip};
use crate::canvas::arrow::components::{ArrowMode, ArrowType};
pub fn add_arrow(
    commands: &mut Commands,
    theme: &Res<Theme>,
    asset_server: &Res<AssetServer>,
    arrow_mode: ArrowMode,
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
    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(3.)),
                width: Val::Percent(13.),
                height: Val::Percent(100.),
                ..default()
            },
            background_color: theme.shadow.into(),
            ..default()
        })
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                background_color: theme.arrow_btn_bg.into(),
                border_color: theme.btn_border.into(),
                image: image.into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    left: Val::Px(1.),
                    right: Val::Px(0.),
                    top: Val::Px(-1.),
                    bottom: Val::Px(0.),
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            arrow_mode,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((
                get_tooltip(
                    theme,
                    text.to_string(),
                    crate::ui_plugin::ui_helpers::TooltipSize::Large,
                    TooltipPosition::Bottom,
                ),
                Tooltip,
            ));
        })
        .id();
    commands.entity(top).add_child(button);
    top
}

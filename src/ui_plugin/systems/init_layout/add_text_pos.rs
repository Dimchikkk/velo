use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::{themes::Theme, ui_plugin::ui_helpers::GenericButton};

use super::ui_helpers::TextPosMode;

pub fn add_text_pos(
    commands: &mut Commands,
    theme: &Res<Theme>,
    arrow_server: &Res<AssetServer>,
    text_pos_mode: TextPosMode,
) -> Entity {
    let image = match text_pos_mode.text_pos {
        crate::TextPos::Center => arrow_server.load("text-center.png"),
        crate::TextPos::TopLeft => arrow_server.load("text-left-top.png"),
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
            background_color: theme.shadow.into(),
            ..default()
        })
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                background_color: theme.text_pos_btn_bg.into(),
                image: image.into(),
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(1.),
                        right: Val::Px(0.),
                        top: Val::Px(-1.),
                        bottom: Val::Px(0.),
                    },
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            BorderColor(theme.btn_border),
            text_pos_mode,
            GenericButton,
        ))
        .id();
    commands.entity(top).add_child(button);
    top
}

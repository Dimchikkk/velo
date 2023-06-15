use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::{themes::Theme, ui_plugin::ui_helpers::GenericButton};

use super::ui_helpers::ChangeColor;

pub fn add_color(commands: &mut Commands, theme: &Res<Theme>, color: Color) -> Entity {
    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(5.)),
                size: Size::new(Val::Percent(20.), Val::Percent(100.)),
                ..default()
            },
            background_color: theme.shadow.into(),
            ..default()
        })
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                background_color: color.into(),
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
            ChangeColor { color },
            GenericButton,
        ))
        .id();
    commands.entity(top).add_child(button);
    top
}

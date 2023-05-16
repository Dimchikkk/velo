use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::chart_plugin::ui_helpers::GenericButton;

use super::ui_helpers::ChangeColor;

pub fn add_color(commands: &mut Commands, color: Color) -> Entity {
    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(5.)),
               width: Val::Percent(20.),
                ..default()
            },
            background_color: Color::BLACK.with_a(0.5).into(),
            ..default()
        })
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                background_color: color.into(),
                style: Style {
                    width:Val::Percent(100.),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,

                        left: Val::Px(-2.),
                        right: Val::Px(0.),
                        top: Val::Px(-2.),
                        bottom: Val::Px(0.),

                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            ChangeColor { color },
            GenericButton,
        ))
        .id();
    commands.entity(top).add_child(button);
    top
}

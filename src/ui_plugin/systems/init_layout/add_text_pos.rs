use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::ui_plugin::ui_helpers::GenericButton;

use super::ui_helpers::TextPosMode;

pub fn add_text_pos(
    commands: &mut Commands,
    arrow_server: &Res<AssetServer>,
    text_pos_mode: TextPosMode,
) -> Entity {
    let image = match text_pos_mode.text_pos {
        crate::TextPos::Center => arrow_server.load("text-center.png"),
        crate::TextPos::BottomRight => arrow_server.load("text-right-bottom.png"),
        crate::TextPos::BottomLeft => arrow_server.load("text-left-bottom.png"),
        crate::TextPos::TopRight => arrow_server.load("text-right-top.png"),
        crate::TextPos::TopLeft => arrow_server.load("text-left-top.png"),
    };
    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(5.)),
                width:Val::Percent(12.), 
                height:Val::Percent(100.),
                ..default()
            },
            background_color: Color::BLACK.with_a(0.5).into(),
            ..default()
        })
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0).into(),
                image: image.into(),
                style: Style {
                    width: Val::Percent(100.),height:Val::Percent(100.),
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
            text_pos_mode,
            GenericButton,
        ))
        .id();
    commands.entity(top).add_child(button);
    top
}

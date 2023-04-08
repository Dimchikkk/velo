use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

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
            text_pos_mode,
        ))
        .id()
}

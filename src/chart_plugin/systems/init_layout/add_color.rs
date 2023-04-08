use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use super::ui_helpers::ChangeColor;

pub fn add_color(commands: &mut Commands, color: Color) -> Entity {
    commands
        .spawn((
            ButtonBundle {
                background_color: color.into(),
                style: Style {
                    size: Size::new(Val::Percent(20.), Val::Percent(100.)),
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
            ChangeColor { color },
        ))
        .id()
}

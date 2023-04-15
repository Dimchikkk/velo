use bevy::prelude::*;

use crate::chart_plugin::ui_helpers::GenericButton;

use super::ui_helpers::create_rectangle_txt;

pub fn add_menu_button(
    commands: &mut Commands,
    font: Handle<Font>,
    name: String,
    component: impl Component,
) -> Entity {
    commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(0.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    padding: UiRect {
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        top: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    align_items: AlignItems::Center,
                    // overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            component,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn(create_rectangle_txt(font.clone(), name, None));
        })
        .id()
}

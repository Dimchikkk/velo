use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::chart_plugin::ui_helpers::{get_tooltip, GenericButton, Tooltip};

use super::ui_helpers::create_rectangle_txt;

pub fn add_menu_button(
    commands: &mut Commands,
    aseet_server: &Res<AssetServer>,
    font: Handle<Font>,
    name: String,
    component: impl Component,
) -> Entity {
    if name == "Save" {
        let (image, text) = (aseet_server.load("save.png"), "Save");
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
                        size: Size::new(Val::Px(30.), Val::Px(30.)),
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.)),
                        margin: UiRect {
                            left: Val::Px(5.),
                            right: Val::Px(5.),
                            top: Val::Px(5.),
                            bottom: Val::Px(5.),
                        },
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                BorderColor(Color::BLACK),
                component,
                GenericButton,
            ))
            .with_children(|builder| {
                builder.spawn((get_tooltip(font, text.to_string(), 14.), Tooltip));
            })
            .id()
    } else {
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
}

use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::chart_plugin::ui_helpers::GenericButton;

use super::ui_helpers::{get_tooltip, ButtonAction, ButtonTypes, Tooltip};

pub fn add_front_back(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    button_action: ButtonAction,
    font: Handle<Font>,
) -> Entity {
    let (image, text) = if button_action.button_type == ButtonTypes::Front {
        (asset_server.load("front.png"), "Move to front")
    } else {
        (asset_server.load("back.png"), "Move to back")
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
            button_action,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((get_tooltip(font, text.to_string(), 14.), Tooltip));
        })
        .id()
}

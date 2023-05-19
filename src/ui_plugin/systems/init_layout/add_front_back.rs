use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::ui_plugin::ui_helpers::{GenericButton, TooltipPosition};

use super::ui_helpers::{get_tooltip, ButtonAction, ButtonTypes, Tooltip};

pub fn add_front_back(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    button_action: ButtonAction,
) -> Entity {
    let (image, text) = if button_action.button_type == ButtonTypes::Front {
        (asset_server.load("front.png"), "Move to front")
    } else {
        (asset_server.load("back.png"), "Move to back")
    };
    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(5.)),
                width:Val::Percent(15.),
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
                   width: Val::Percent(100.), 
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.)),
                    position_type: PositionType::Absolute,
             
                        left: Val::Px(-2.),
                        right: Val::Px(0.),
                        top: Val::Px(-2.),
                        bottom: Val::Px(0.),
            
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
            builder.spawn((
                get_tooltip(text.to_string(), 14., TooltipPosition::Bottom),
                Tooltip,
            ));
        })
        .id();
    commands.entity(top).add_child(button);
    top
}

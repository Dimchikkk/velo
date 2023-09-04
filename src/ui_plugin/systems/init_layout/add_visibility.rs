use bevy::{prelude::*, text::BreakLineOn};

use crate::{
    themes::Theme,
    ui_plugin::ui_helpers::{get_tooltip, ButtonAction, GenericButton, Tooltip, TooltipPosition},
};

pub fn add_visibility(
    commands: &mut Commands,
    theme: &Res<Theme>,
    button_action: ButtonAction,
    tooltip_label: String,
    icon_font: &Handle<Font>,
) -> Entity {
    let icon_code = match button_action.button_type {
        crate::ui_plugin::ui_helpers::ButtonTypes::ShowChildren => "\u{e8f4}".to_string(),
        crate::ui_plugin::ui_helpers::ButtonTypes::HideChildren => "\u{e8f5}".to_string(),
        crate::ui_plugin::ui_helpers::ButtonTypes::ShowRandom => "\u{e043}".to_string(),
        _ => panic!("unexpected button type"),
    };

    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(5.)),
                width: Val::Percent(15.),
                height: Val::Percent(100.),
                ..default()
            },
            background_color: theme.shadow.into(),
            ..default()
        })
        .id();
    let new_button_action = commands
        .spawn((
            ButtonBundle {
                background_color: Color::BLACK.into(),
                border_color: theme.btn_border.into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    left: Val::Px(1.),
                    right: Val::Px(0.),
                    top: Val::Px(-1.),
                    bottom: Val::Px(0.),
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            button_action,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((
                get_tooltip(theme, tooltip_label, TooltipPosition::Bottom),
                Tooltip,
            ));

            let text_style = TextStyle {
                font_size: 25.0,
                color: theme.text_pos_btn_bg,
                font: icon_font.clone(),
            };
            let text = Text {
                sections: vec![TextSection {
                    value: icon_code,
                    style: text_style,
                }],
                alignment: TextAlignment::Left,
                linebreak_behavior: BreakLineOn::WordBoundary,
            };
            let text_bundle_style = Style {
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::all(Val::Px(3.)),
                ..default()
            };

            builder.spawn(TextBundle {
                text,
                style: text_bundle_style,
                ..default()
            });
        })
        .id();
    commands.entity(top).add_child(new_button_action);
    top
}

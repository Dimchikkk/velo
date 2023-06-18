use bevy::{prelude::*, text::BreakLineOn};

use crate::{themes::Theme, ui_plugin::ui_helpers::GenericButton};

pub fn add_effect(
    commands: &mut Commands,
    theme: &Res<Theme>,
    icon_font: &Handle<Font>,
    component: impl Component,
) -> Entity {
    let top = commands
        .spawn((NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(5.)),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                width: Val::Percent(2.3),
                height: Val::Percent(85.),
                ..default()
            },
            ..default()
        },))
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                background_color: theme.celebrate_btn_bg.into(),
                style: Style {
                    padding: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(0.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            component,
            GenericButton,
        ))
        .with_children(|builder| {
            let text_style = TextStyle {
                font_size: 25.0,
                color: theme.celebrate_btn,
                font: icon_font.clone(),
            };
            let text = Text {
                sections: vec![TextSection {
                    value: "\u{ea65}".to_string(),
                    style: text_style,
                }],
                alignment: TextAlignment::Left,
                linebreak_behavior: BreakLineOn::WordBoundary,
            };

            builder.spawn(TextBundle { text, ..default() });
        })
        .id();
    commands.entity(top).add_child(button);
    top
}

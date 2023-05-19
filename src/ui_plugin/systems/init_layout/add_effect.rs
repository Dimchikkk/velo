use bevy::{prelude::*, text::BreakLineOn};

use crate::ui_plugin::ui_helpers::GenericButton;

pub fn add_effect(
    commands: &mut Commands,
    icon_font: &Handle<Font>,
    component: impl Component,
) -> Entity {
    let top = commands
        .spawn((NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(5.)),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                width:Val::Percent(2.3),height: Val::Percent(85.),
                ..default()
            },
            ..default()
        },))
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0).into(),
                style: Style {
                    padding: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(0.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    width:Val::Percent(100.), height:Val::Percent(100.),
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
                color: Color::RED,
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

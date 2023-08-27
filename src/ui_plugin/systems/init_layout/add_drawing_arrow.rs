use bevy::{prelude::*, text::BreakLineOn};

use crate::{themes::Theme, ui_plugin::ui_helpers::GenericButton};

pub fn add_drawing_arrow(
    commands: &mut Commands,
    theme: &Res<Theme>,
    icon_font: &Handle<Font>,
    component: impl Component + Clone,
) -> Entity {
    let top = commands
        .spawn((NodeBundle {
            style: Style {
                align_self: AlignSelf::Stretch,
                flex_direction: FlexDirection::Column,
                margin: UiRect {
                    left: Val::Px(8.),
                    right: Val::Px(20.),
                    ..default()
                },
                padding: UiRect {
                    top: Val::Px(3.),
                    ..default()
                },
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
                background_color: theme.drawing_arrow_btn_bg.into(),
                style: Style {
                    padding: UiRect::all(Val::Px(10.)),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            component.clone(),
            GenericButton,
        ))
        .with_children(|builder| {
            let text_style = TextStyle {
                font_size: 25.0,
                color: theme.drawing_arrow_btn.with_a(0.5),
                font: icon_font.clone(),
            };
            let text = Text {
                sections: vec![TextSection {
                    value: "\u{f8ce}".to_string(),
                    style: text_style,
                }],
                alignment: TextAlignment::Center,
                linebreak_behavior: BreakLineOn::WordBoundary,
            };

            builder.spawn((TextBundle { text, ..default() }, component));
        })
        .id();
    commands.entity(top).add_child(button);
    top
}

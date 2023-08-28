use bevy::{prelude::*, text::BreakLineOn};

use crate::{
    themes::Theme,
    ui_plugin::ui_helpers::{GenericButton, TwoPointsDraw},
};

pub fn add_two_points_draw(
    commands: &mut Commands,
    theme: &Res<Theme>,
    icon_font: &Handle<Font>,
    component: TwoPointsDraw,
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
    let code = match component.drawing_type {
        crate::ui_plugin::ui_helpers::TwoPointsDrawType::Arrow => "\u{f8ce}",
        crate::ui_plugin::ui_helpers::TwoPointsDrawType::Line => "\u{f108}",
        crate::ui_plugin::ui_helpers::TwoPointsDrawType::Rhombus => "\u{e418}",
        crate::ui_plugin::ui_helpers::TwoPointsDrawType::Square => "\u{e3c1}",
    };
    let button = commands
        .spawn((
            ButtonBundle {
                background_color: theme.drawing_two_points_btn_bg.into(),
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
                color: theme.drawing_two_points_btn.with_a(0.5),
                font: icon_font.clone(),
            };
            let text = Text {
                sections: vec![TextSection {
                    value: code.to_string(),
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

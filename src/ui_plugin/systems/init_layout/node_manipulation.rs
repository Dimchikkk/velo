use bevy::{prelude::*, text::BreakLineOn};

use crate::ui_plugin::ui_helpers::{
    get_tooltip, ButtonAction, GenericButton, Tooltip, TooltipPosition,
};

pub fn node_manipulation(
    commands: &mut Commands,
    icon_font: &Handle<Font>,
    create_rec_component: ButtonAction,
    create_circle_component: ButtonAction,
    delete_component: ButtonAction,
) -> Entity {
    let node = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                width: Val::Percent(90.),
                height: Val::Percent(14.),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        .id();
    let top_new_rec = commands
        .spawn(NodeBundle {
            background_color: Color::BLACK.with_a(0.5).into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(5.)),
                width: Val::Percent(23.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .id();
    let new_rec = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0).into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,

                    left: Val::Px(-2.),
                    right: Val::Px(0.),
                    top: Val::Px(-2.),
                    bottom: Val::Px(0.),

                    ..default()
                },
                ..default()
            },
            create_rec_component,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((
                get_tooltip("New Rectangle".to_string(), 14., TooltipPosition::Bottom),
                Tooltip,
            ));

            let text_style = TextStyle {
                font_size: 30.0,
                color: Color::BLACK,
                font: icon_font.clone(),
            };
            let text = Text {
                sections: vec![TextSection {
                    value: "\u{eb54}".to_string(),
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
    let top_new_circle = commands
        .spawn(NodeBundle {
            background_color: Color::BLACK.with_a(0.5).into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(5.)),
                width: Val::Percent(23.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .id();
    let new_circle = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0).into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,

                    left: Val::Px(-2.),
                    right: Val::Px(0.),
                    top: Val::Px(-2.),
                    bottom: Val::Px(0.),

                    ..default()
                },
                ..default()
            },
            create_circle_component,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((
                get_tooltip("New Circle".to_string(), 14., TooltipPosition::Bottom),
                Tooltip,
            ));

            let text_style = TextStyle {
                font_size: 30.0,
                color: Color::BLACK,
                font: icon_font.clone(),
            };
            let text = Text {
                sections: vec![TextSection {
                    value: "\u{ef4a}".to_string(),
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
    let top_del = commands
        .spawn(NodeBundle {
            background_color: Color::BLACK.with_a(0.5).into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(5.)),
                align_self: AlignSelf::Stretch,
                width: Val::Percent(23.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .id();
    let del_rec = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0).into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,

                    left: Val::Px(-2.),
                    right: Val::Px(0.),
                    top: Val::Px(-2.),
                    bottom: Val::Px(0.),

                    ..default()
                },
                ..default()
            },
            delete_component,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((
                get_tooltip("Delete Rectangle".to_string(), 14., TooltipPosition::Bottom),
                Tooltip,
            ));

            let text_style = TextStyle {
                font_size: 30.0,
                color: Color::BLACK,
                font: icon_font.clone(),
            };
            let text = Text {
                sections: vec![TextSection {
                    value: "\u{e872}".to_string(),
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
    commands.entity(top_new_circle).add_child(new_circle);
    commands.entity(top_new_rec).add_child(new_rec);
    commands.entity(top_del).add_child(del_rec);
    commands.entity(node).add_child(top_del);
    commands.entity(node).add_child(top_new_circle);
    commands.entity(node).add_child(top_new_rec);
    node
}

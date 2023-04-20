use bevy::{prelude::*, text::BreakLineOn};
use bevy_ui_borders::BorderColor;

use crate::chart_plugin::ui_helpers::{get_tooltip, GenericButton, Tooltip};

pub fn add_menu_button(
    commands: &mut Commands,
    aseet_server: &Res<AssetServer>,
    name: String,
    component: impl Component,
) -> Entity {
    match name.as_str() {
        "Save" => {
            let (image, text) = (aseet_server.load("save.png"), "Save");
            let top = commands
                .spawn(NodeBundle {
                    background_color: Color::BLACK.with_a(0.5).into(),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        margin: UiRect {
                            left: Val::Px(20.),
                            right: Val::Px(5.),
                            top: Val::Px(3.),
                            bottom: Val::Px(2.),
                        },
                        size: Size::new(Val::Percent(2.5), Val::Percent(90.)),
                        ..default()
                    },
                    ..default()
                })
                .id();
            let button = commands
                .spawn((
                    ButtonBundle {
                        image: image.into(),
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Px(-2.),
                                right: Val::Px(0.),
                                top: Val::Px(-2.),
                                bottom: Val::Px(0.),
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
                    builder.spawn((get_tooltip(text.to_string(), 14.), Tooltip));
                })
                .id();
            commands.entity(top).add_child(button);
            top
        }
        "New Document" | "New Tab" => {
            let top = commands
                .spawn(NodeBundle {
                    background_color: Color::BLACK.with_a(0.5).into(),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        margin: UiRect {
                            left: Val::Px(10.),
                            right: Val::Px(10.),
                            top: Val::Px(3.),
                            bottom: Val::Px(2.),
                        },
                        size: Size::new(Val::Percent(2.5), Val::Percent(90.)),
                        ..default()
                    },
                    ..default()
                })
                .id();
            let button = commands
                .spawn((
                    ButtonBundle {
                        background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Px(-2.),
                                right: Val::Px(0.),
                                top: Val::Px(-2.),
                                bottom: Val::Px(0.),
                            },
                            // overflow: Overflow::Hidden,
                            ..default()
                        },
                        ..default()
                    },
                    component,
                    GenericButton,
                ))
                .with_children(|builder| {
                    builder.spawn((get_tooltip(name, 14.), Tooltip));

                    let text_style = TextStyle {
                        font_size: 64.0,
                        color: Color::BLACK,
                        ..default()
                    };
                    let text = Text {
                        sections: vec![TextSection {
                            value: "+".to_string(),
                            style: text_style,
                        }],
                        alignment: TextAlignment::Left,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
                    };
                    let text_bundle_style = Style {
                        position_type: PositionType::Absolute,
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    };

                    builder.spawn(TextBundle {
                        text,
                        style: text_bundle_style,
                        ..default()
                    });
                })
                .id();
            commands.entity(top).add_child(button);
            top
        }
        _ => panic!("no such button"),
    }
}

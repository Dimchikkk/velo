use bevy::{prelude::*, text::BreakLineOn};
use bevy_ui_borders::BorderColor;

use crate::chart_plugin::ui_helpers::{get_tooltip, GenericButton, Tooltip};

pub fn add_menu_button(
    commands: &mut Commands,
    label: String,
    icon_font: &Handle<Font>,
    component: impl Component,
) -> Entity {
    let icon_code = match label.as_str() {
        "New Tab" => "\u{e3ba}",
        "New Document" => "\u{e89c}",
        "Save Document" => "\u{e161}",
        "Export To File" => "\u{e2c6}",
        "Import From File" => "\u{e255}",
        "Import From URL" => "\u{e902}",
        "Save Document to window.velo object" => "\u{e866}",
        "Share Document (copy URL to clipboard)" => "\u{e80d}",
        _ => panic!("Unknown menu button tooltip label: {}", label),
    };
    match label.as_str() {
        "New Tab" => {
            let top = commands
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        margin: UiRect {
                            left: Val::Px(10.),
                            right: Val::Px(10.),
                            top: Val::Px(3.),
                            bottom: Val::Px(3.),
                        },
                        size: Size::new(Val::Percent(2.3), Val::Percent(85.)),
                        ..default()
                    },
                    ..default()
                },))
                .id();
            let button = commands
                .spawn((
                    ButtonBundle {
                        background_color: Color::rgb(189.0 / 255.0, 189.0 / 255.0, 189.0 / 255.0)
                            .into(),
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
                    builder.spawn((get_tooltip(label, 14.), Tooltip));

                    let text_style = TextStyle {
                        font_size: 30.0,
                        color: Color::BLACK,
                        font: icon_font.clone(),
                    };
                    let text = Text {
                        sections: vec![TextSection {
                            value: icon_code.to_string(),
                            style: text_style,
                        }],
                        alignment: TextAlignment::Left,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
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
            commands.entity(top).add_child(button);
            top
        }
        _ => {
            let top = commands
                .spawn((
                    NodeBundle {
                        background_color: Color::BLACK.with_a(0.1).into(),
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::Stretch,
                            border: UiRect::all(Val::Px(1.)),
                            margin: UiRect {
                                left: Val::Px(10.),
                                right: Val::Px(10.),
                                top: Val::Px(3.),
                                bottom: Val::Px(3.),
                            },
                            size: Size::new(Val::Percent(2.3), Val::Percent(85.)),
                            ..default()
                        },
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                ))
                .id();
            let button = commands
                .spawn((
                    ButtonBundle {
                        background_color: Color::rgb(0.0 / 255.0, 150.0 / 255.0, 136.0 / 255.0)
                            .into(),
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
                    builder.spawn((get_tooltip(label, 14.), Tooltip));

                    let text_style = TextStyle {
                        font_size: 30.0,
                        color: Color::BLACK,
                        font: icon_font.clone(),
                    };
                    let text = Text {
                        sections: vec![TextSection {
                            value: icon_code.to_string(),
                            style: text_style,
                        }],
                        alignment: TextAlignment::Left,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
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
            commands.entity(top).add_child(button);
            top
        }
    }
}

use bevy::{prelude::*, text::BreakLineOn};
use bevy_ui_borders::BorderColor;

use crate::{
    themes::Theme,
    ui_plugin::ui_helpers::{get_tooltip, GenericButton, Tooltip, TooltipPosition},
};

pub fn add_menu_button(
    commands: &mut Commands,
    theme: &Res<Theme>,
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
                        align_self: AlignSelf::Stretch,
                        flex_direction: FlexDirection::Column,
                        margin: UiRect {
                            left: Val::Px(10.),
                            right: Val::Px(10.),
                            ..default()
                        },
                        padding: UiRect {
                            top: Val::Px(3.),
                            ..default()
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
                        background_color: theme.new_tab_btn_bg.into(),
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
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
                        font_size: 30.0,
                        color: theme.menu_btn,
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

                    builder.spawn(TextBundle { text, ..default() });
                })
                .id();
            commands.entity(top).add_child(button);
            top
        }
        _ => {
            let top = commands
                .spawn((
                    NodeBundle {
                        background_color: theme.shadow.into(),
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
                    BorderColor(theme.btn_border),
                ))
                .id();
            let button = commands
                .spawn((
                    ButtonBundle {
                        background_color: theme.menu_btn_bg.into(),
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Px(1.),
                                right: Val::Px(0.),
                                top: Val::Px(-3.),
                                bottom: Val::Px(0.),
                            },
                            ..default()
                        },
                        ..default()
                    },
                    component,
                    GenericButton,
                ))
                .with_children(|builder| {
                    builder.spawn((
                        get_tooltip(theme, label, 14., TooltipPosition::Bottom),
                        Tooltip,
                    ));

                    let text_style = TextStyle {
                        font_size: 30.0,
                        color: theme.menu_btn,
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

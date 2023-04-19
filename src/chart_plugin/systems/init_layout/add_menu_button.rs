use bevy::{prelude::*, text::BreakLineOn};
use bevy_ui_borders::BorderColor;

use crate::chart_plugin::ui_helpers::{get_tooltip, GenericButton, Tooltip};

pub fn add_menu_button(
    commands: &mut Commands,
    aseet_server: &Res<AssetServer>,
    font: Handle<Font>,
    name: String,
    component: impl Component,
) -> Entity {
    match name.as_str() {
        "Save" => {
            let (image, text) = (aseet_server.load("save.png"), "Save");
            commands
                .spawn((
                    ButtonBundle {
                        background_color: Color::Rgba {
                            red: 1.,
                            green: 1.,
                            blue: 1.,
                            alpha: 0.5,
                        }
                        .into(),
                        image: image.into(),
                        style: Style {
                            size: Size::new(Val::Px(30.), Val::Px(30.)),
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.)),
                            margin: UiRect {
                                left: Val::Px(5.),
                                right: Val::Px(5.),
                                top: Val::Px(5.),
                                bottom: Val::Px(5.),
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
                    builder.spawn((get_tooltip(font, text.to_string(), 14.), Tooltip));
                })
                .id()
        }
        "New Document" | "New Tab" => {
            commands
                .spawn((
                    ButtonBundle {
                        background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                        style: Style {
                            size: Size::new(Val::Px(30.), Val::Px(30.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            // overflow: Overflow::Hidden,
                            ..default()
                        },
                        ..default()
                    },
                    component,
                    GenericButton,
                ))
                .with_children(|builder| {
                    builder.spawn((get_tooltip(font.clone(), name, 14.), Tooltip));

                    let text_style = TextStyle {
                        font: font.clone(),
                        font_size: 64.0,
                        color: Color::GRAY,
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
                .id()
        }
        _ => panic!("no such button"),
    }
}

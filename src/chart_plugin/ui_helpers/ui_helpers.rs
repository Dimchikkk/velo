use bevy_ui_borders::{BorderColor, Outline};
use linkify::{LinkFinder, LinkKind};

use std::f32::consts::PI;

use bevy::{prelude::*, text::BreakLineOn};
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, ShapeBundle, Stroke},
    shapes,
};

use crate::TextPos;

#[path = "components.rs"]
mod components;
pub use components::*;

fn get_marker_style(position: UiRect, size: f32) -> Style {
    Style {
        position_type: PositionType::Absolute,
        position,
        border: UiRect::all(Val::Px(1.)),
        size: Size::new(Val::Px(size), Val::Px(size)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn add_rectangle_txt(font: Handle<Font>, text: String) -> TextBundle {
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::BLACK,
    };
    TextBundle::from_section(text, text_style).with_style(Style {
        position_type: PositionType::Relative,
        ..default()
    })
}

pub fn pos_to_style(text_pos: TextPos) -> (JustifyContent, AlignItems) {
    match text_pos {
        TextPos::TopRight => (JustifyContent::FlexEnd, AlignItems::FlexStart),
        TextPos::TopLeft => (JustifyContent::FlexStart, AlignItems::FlexStart),
        TextPos::BottomRight => (JustifyContent::FlexEnd, AlignItems::FlexEnd),
        TextPos::BottomLeft => (JustifyContent::FlexStart, AlignItems::FlexEnd),
        TextPos::Center => (JustifyContent::Center, AlignItems::Center),
    }
}
pub fn style_to_pos(style: (JustifyContent, AlignItems)) -> TextPos {
    match style {
        (JustifyContent::FlexEnd, AlignItems::FlexStart) => TextPos::TopRight,
        (JustifyContent::FlexStart, AlignItems::FlexStart) => TextPos::TopLeft,
        (JustifyContent::FlexEnd, AlignItems::FlexEnd) => TextPos::BottomRight,
        (JustifyContent::FlexStart, AlignItems::FlexEnd) => TextPos::BottomLeft,
        (JustifyContent::Center, AlignItems::Center) => TextPos::Center,
        _ => TextPos::Center,
    }
}

fn create_rectangle_btn(
    size: (Val, Val),
    position: (Val, Val),
    bg_color: Color,
    image: Option<UiImage>,
    z_index: i32,
    text_pos: TextPos,
) -> ButtonBundle {
    let (justify_content, align_items) = pos_to_style(text_pos);
    let mut button = ButtonBundle {
        background_color: bg_color.into(),
        z_index: ZIndex::Local(z_index),
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: position.0,
                bottom: position.1,
                ..Default::default()
            },
            size: Size::new(size.0, size.1),
            justify_content,
            align_items,
            // overflow: Overflow::Hidden,
            ..default()
        },
        ..default()
    };
    if let Some(image) = image {
        button.image = image;
    }
    button
}

fn create_arrow_marker(left: f32, right: f32, top: f32, bottom: f32) -> ButtonBundle {
    ButtonBundle {
        style: get_marker_style(
            UiRect {
                left: Val::Percent(left),
                right: Val::Percent(right),
                top: Val::Percent(top),
                bottom: Val::Percent(bottom),
            },
            4.,
        ),
        ..default()
    }
}

fn create_resize_marker(left: f32, right: f32, top: f32, bottom: f32) -> ButtonBundle {
    ButtonBundle {
        style: get_marker_style(
            UiRect {
                left: Val::Percent(left),
                right: Val::Percent(right),
                top: Val::Percent(top),
                bottom: Val::Percent(bottom),
            },
            10.,
        ),
        background_color: Color::rgba(0., 0., 0., 0.).into(),
        ..default()
    }
}

pub fn get_sections(text: String, font: Handle<Font>) -> Vec<TextSection> {
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 18.0,
        color: Color::BLACK,
    };
    let link_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::BLUE,
    };
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let links: Vec<_> = finder.links(&text).collect();
    if links.is_empty() {
        return vec![
            (TextSection {
                value: text,
                style: text_style,
            }),
        ];
    }
    let mut sections = vec![];
    let mut idx = 0;
    for link in links {
        let start = link.start();
        let end = link.end();
        if start > idx {
            sections.push(TextSection {
                value: text[idx..start].to_string(),
                style: text_style.clone(),
            });
        }
        sections.push(TextSection {
            value: text[start..end].to_string(),
            style: link_style.clone(),
        });
        idx = end;
    }
    if idx < text.len() {
        sections.push(TextSection {
            value: text[idx..text.len()].to_string(),
            style: text_style,
        });
    }
    sections
}

pub fn create_rectangle_txt(
    font: Handle<Font>,
    text: String,
    max_size: Option<(Val, Val)>,
) -> TextBundle {
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::BLACK,
    };
    let text = Text {
        sections: vec![TextSection {
            value: text,
            style: text_style,
        }],
        alignment: TextAlignment::Left,
        linebreak_behaviour: BreakLineOn::WordBoundary,
    };
    let mut text_bundle_style = Style {
        padding: UiRect {
            left: Val::Px(5.),
            right: Val::Px(5.),
            top: Val::Px(5.),
            bottom: Val::Px(5.),
        },
        ..default()
    };
    if max_size.is_some() {
        text_bundle_style.max_size = Size::new(max_size.unwrap().0, max_size.unwrap().1);
    }
    TextBundle {
        text,
        style: text_bundle_style,
        ..default()
    }
}

pub fn spawn_path_modal(
    commands: &mut Commands,
    font: Handle<Font>,
    id: ReflectableUuid,
    save: bool,
) -> Entity {
    let width = 300.;
    let height = 200.;
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(1),
                background_color: Color::WHITE.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(width), Val::Px(height)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            PathModalTop { id },
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(50.),
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder: &mut ChildBuilder| {
                    builder
                        .spawn(ButtonBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                // overflow: Overflow::Hidden,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn(add_rectangle_txt(
                                font.clone(),
                                "Enter file name:".to_string(),
                            ));
                        });
                    builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.)),
                                    padding: UiRect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        top: Val::Px(5.),
                                        bottom: Val::Px(5.),
                                    },
                                    // overflow: Overflow::Hidden,
                                    ..default()
                                },
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            PathModalText { id, save },
                        ))
                        .with_children(|builder| {
                            builder.spawn((
                                create_rectangle_txt(
                                    font.clone(),
                                    "./data/rusticify.json".to_string(),
                                    None,
                                ),
                                PathModalTextInput { id, save },
                            ));
                        });
                });

            builder
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(50.),
                        },
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    border: UiRect::all(Val::Px(1.)),
                                    align_items: AlignItems::Center,
                                    padding: UiRect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        top: Val::Px(5.),
                                        bottom: Val::Px(5.),
                                    },
                                    // overflow: Overflow::Hidden,
                                    ..default()
                                },
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            PathModalConfirm { id, save },
                        ))
                        .with_children(|builder| {
                            builder.spawn(add_rectangle_txt(
                                font.clone(),
                                if save {
                                    "Save".to_string()
                                } else {
                                    "Load".to_string()
                                },
                            ));
                        });
                    builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.)),
                                    padding: UiRect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        top: Val::Px(5.),
                                        bottom: Val::Px(5.),
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            PathModalCancel { id },
                        ))
                        .with_children(|builder| {
                            builder.spawn(add_rectangle_txt(font.clone(), "Cancel".to_string()));
                        });
                });
        })
        .id()
}

#[derive(Clone)]
pub struct NodeMeta {
    pub id: ReflectableUuid,
    pub size: (Val, Val),
    pub position: (Val, Val),
    pub text: String,
    pub bg_color: Color,
    pub font: Handle<Font>,
    pub image: Option<UiImage>,
    pub tags: Vec<String>,
    pub text_pos: TextPos,
    pub z_index: i32,
}

pub fn spawn_node(commands: &mut Commands, item_meta: NodeMeta) -> Entity {
    commands
        .spawn((
            create_rectangle_btn(
                item_meta.size,
                item_meta.position,
                item_meta.bg_color,
                item_meta.image,
                item_meta.z_index,
                item_meta.text_pos,
            ),
            Rectangle { id: item_meta.id },
            Outline::all(Color::BLACK, Val::Px(1.)),
        ))
        .with_children(|builder| {
            builder.spawn((
                create_arrow_marker(50.0, 0., 0., 0.),
                BorderColor(Color::BLACK),
                ArrowConnect {
                    pos: ArrowConnectPos::Top,
                    id: item_meta.id,
                },
            ));
            builder.spawn((
                create_arrow_marker(0., 0., 50., 0.),
                BorderColor(Color::BLACK),
                ArrowConnect {
                    pos: ArrowConnectPos::Left,
                    id: item_meta.id,
                },
            ));
            builder.spawn((
                create_arrow_marker(50., 0., 100., 0.),
                BorderColor(Color::BLACK),
                ArrowConnect {
                    pos: ArrowConnectPos::Bottom,
                    id: item_meta.id,
                },
            ));
            builder.spawn((
                create_arrow_marker(100., 0., 50., 0.),
                BorderColor(Color::BLACK),
                ArrowConnect {
                    pos: ArrowConnectPos::Right,
                    id: item_meta.id,
                },
            ));
            builder.spawn((create_resize_marker(0., 0., 0., 0.), ResizeMarker::TopLeft));
            builder.spawn((
                create_resize_marker(100., 0., 0., 0.),
                ResizeMarker::TopRight,
            ));
            builder.spawn((
                create_resize_marker(100., 0., 100., 0.),
                ResizeMarker::BottomRight,
            ));
            builder.spawn((
                create_resize_marker(0., 0., 100., 0.),
                ResizeMarker::BottomLeft,
            ));
            builder.spawn((
                create_rectangle_txt(item_meta.font, item_meta.text, Some(item_meta.size)),
                EditableText { id: item_meta.id },
            ));
        })
        .id()
}

pub fn create_arrow(commands: &mut Commands, start: Vec2, end: Vec2, arrow_meta: ArrowMeta) {
    match arrow_meta.arrow_type {
        ArrowType::Line => {
            let main = shapes::Line(start, end);
            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&main),
                    ..default()
                },
                arrow_meta,
                Stroke::new(Color::BLACK, 2.0),
            ));
        }
        ArrowType::Arrow => {
            let headlen = 10.0;
            let main = shapes::Line(start, end);
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            let part_one = shapes::Line(
                end,
                Vec2::new(
                    end.x - headlen * (angle - PI / 6.).cos(),
                    end.y - headlen * (angle - PI / 6.).sin(),
                ),
            );
            let part_two = shapes::Line(
                end,
                Vec2::new(
                    end.x - headlen * (angle + PI / 6.).cos(),
                    end.y - headlen * (angle + PI / 6.).sin(),
                ),
            );
            commands
                .spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&main),
                        ..default()
                    },
                    arrow_meta,
                    Stroke::new(Color::BLACK, 2.0),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&part_one),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                    builder.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&part_two),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                });
        }
        ArrowType::DoubleArrow => {
            let headlen = 10.0;
            let main = shapes::Line(start, end);
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            let part_one = shapes::Line(
                end,
                Vec2::new(
                    end.x - headlen * (angle - PI / 6.).cos(),
                    end.y - headlen * (angle - PI / 6.).sin(),
                ),
            );
            let part_two = shapes::Line(
                end,
                Vec2::new(
                    end.x - headlen * (angle + PI / 6.).cos(),
                    end.y - headlen * (angle + PI / 6.).sin(),
                ),
            );
            let part_three = shapes::Line(
                start,
                Vec2::new(
                    start.x + headlen * (angle - PI / 6.).cos(),
                    start.y + headlen * (angle - PI / 6.).sin(),
                ),
            );
            let part_four = shapes::Line(
                start,
                Vec2::new(
                    start.x + headlen * (angle + PI / 6.).cos(),
                    start.y + headlen * (angle + PI / 6.).sin(),
                ),
            );
            commands
                .spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&main),
                        ..default()
                    },
                    arrow_meta,
                    Stroke::new(Color::BLACK, 2.0),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&part_one),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                    builder.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&part_two),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                    builder.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&part_three),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                    builder.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&part_four),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                });
        }
        ArrowType::ParallelLine => {
            eprintln!("Parallel line is not implemented yet")
        }
        ArrowType::ParallelArrow => {
            eprintln!("Parallel line is not implemented yet")
        }
        ArrowType::ParallelDoubleArrow => {
            eprintln!("Parallel Double Arrow is not implemented yet")
        }
    }
}

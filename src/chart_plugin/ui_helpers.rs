use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, ShapeBundle, Stroke},
    shapes,
};
use moonshine_save::save::Save;
use uuid::Uuid;

#[derive(Component)]
pub struct MainCamera;

#[derive(Clone, Reflect, Default, Debug, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[reflect_value]
pub struct ReflectableUuid(pub Uuid);

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct Rectangle {
    pub id: ReflectableUuid,
}

#[derive(Component, Default)]
pub struct CreateRectButton;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EditableText {
    pub id: ReflectableUuid,
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Top {
    pub id: ReflectableUuid,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Reflect, Default)]
pub enum ArrowConnectPos {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Reflect, Default)]
#[reflect(Component)]
pub struct ArrowConnect {
    pub id: ReflectableUuid,
    pub pos: ArrowConnectPos,
}

#[derive(Component, Copy, Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub enum ResizeMarker {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Reflect, Default)]
#[reflect(Component)]
pub struct ArrowMeta {
    pub start: ArrowConnect,
    pub end: ArrowConnect,
}

fn get_marker_style(position: UiRect) -> Style {
    Style {
        position_type: PositionType::Absolute,
        position,
        size: Size::new(Val::Px(5.), Val::Px(5.)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn add_rectangle_btn() -> ButtonBundle {
    ButtonBundle {
        z_index: ZIndex::Global(1),
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..Default::default()
            },
            size: Size::new(Val::Px(100.), Val::Px(100.)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

pub fn add_rectangle_txt(font: Handle<Font>) -> TextBundle {
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::BLACK,
    };
    TextBundle::from_section("NEW RECT", text_style).with_style(Style {
        position_type: PositionType::Relative,
        ..default()
    })
}

fn create_rectangle_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..Default::default()
            },
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }
}

fn create_rectangle_btn(size: Vec2, image: Option<UiImage>) -> ButtonBundle {
    let mut button = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(size.x), Val::Px(size.y)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
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
        style: get_marker_style(UiRect {
            left: Val::Percent(left),
            right: Val::Percent(right),
            top: Val::Percent(top),
            bottom: Val::Percent(bottom),
        }),
        background_color: Color::rgb(0.9, 0.9, 1.0).into(),
        ..default()
    }
}

fn create_resize_marker(left: f32, right: f32, top: f32, bottom: f32) -> ButtonBundle {
    ButtonBundle {
        style: get_marker_style(UiRect {
            left: Val::Percent(left),
            right: Val::Percent(right),
            top: Val::Percent(top),
            bottom: Val::Percent(bottom),
        }),
        background_color: Color::rgb(0.8, 0.8, 1.0).into(),
        ..default()
    }
}

fn create_rectangle_txt(font: Handle<Font>) -> TextBundle {
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::BLACK,
    };
    TextBundle {
        text: Text::from_section("", text_style),
        style: Style {
            position_type: PositionType::Relative,
            ..default()
        },
        ..default()
    }
}

pub struct ItemMeta {
    pub id: ReflectableUuid,
    pub size: Vec2,
    pub font: Handle<Font>,
    pub image: Option<UiImage>,
}

pub fn spawn_item(commands: &mut Commands, item_meta: ItemMeta) {
    commands
        .spawn((create_rectangle_node(), Top { id: item_meta.id }, Save))
        .with_children(|builder| {
            builder
                .spawn((
                    create_rectangle_btn(item_meta.size, item_meta.image),
                    Rectangle { id: item_meta.id },
                    Save,
                ))
                .with_children(|builder| {
                    builder.spawn((
                        create_arrow_marker(50.0, 0., 0., 0.),
                        ArrowConnect {
                            pos: ArrowConnectPos::Top,
                            id: item_meta.id,
                        },
                        Save,
                    ));
                    builder.spawn((
                        create_arrow_marker(0., 0., 50., 0.),
                        ArrowConnect {
                            pos: ArrowConnectPos::Left,
                            id: item_meta.id,
                        },
                        Save,
                    ));
                    builder.spawn((
                        create_arrow_marker(50., 0., 100., 0.),
                        ArrowConnect {
                            pos: ArrowConnectPos::Bottom,
                            id: item_meta.id,
                        },
                        Save,
                    ));
                    builder.spawn((
                        create_arrow_marker(100., 0., 50., 0.),
                        ArrowConnect {
                            pos: ArrowConnectPos::Right,
                            id: item_meta.id,
                        },
                        Save,
                    ));
                    builder.spawn((
                        create_resize_marker(0., 0., 0., 0.),
                        ResizeMarker::TopLeft,
                        Save,
                    ));
                    builder.spawn((
                        create_resize_marker(100., 0., 0., 0.),
                        ResizeMarker::TopRight,
                        Save,
                    ));
                    builder.spawn((
                        create_resize_marker(100., 0., 100., 0.),
                        ResizeMarker::BottomRight,
                        Save,
                    ));
                    builder.spawn((
                        create_resize_marker(0., 0., 100., 0.),
                        ResizeMarker::BottomLeft,
                        Save,
                    ));
                    builder.spawn((
                        create_rectangle_txt(item_meta.font),
                        EditableText { id: item_meta.id },
                        Save,
                    ));
                });
        });
}

pub fn create_arrow(commands: &mut Commands, start: Vec2, end: Vec2, arrow_meta: ArrowMeta) {
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
            Save,
        ))
        .with_children(|builder| {
            builder.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&part_one),
                    ..default()
                },
                Stroke::new(Color::BLACK, 2.0),
                Save,
            ));
            builder.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&part_two),
                    ..default()
                },
                Stroke::new(Color::BLACK, 2.0),
                Save,
            ));
        });
}

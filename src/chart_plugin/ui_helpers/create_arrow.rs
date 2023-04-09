use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, ShapeBundle, Stroke},
    shapes,
};

use super::{ArrowMeta, ArrowType};

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
            let main = shapes::Line(start, Vec2::new((start.y + end.y) / 2.0, start.y));
            let mid = shapes::Line(
                Vec2::new((start.y + end.y) / 2.0, start.y),
                Vec2::new((start.y + end.y) / 2.0, end.y),
            );
            let main2 = shapes::Line(Vec2::new((start.y + end.y) / 2.0, end.y), end);
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
                            path: GeometryBuilder::build_as(&mid),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                    builder.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&main2),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                });
        }
        ArrowType::ParallelArrow => {
            let headlen = 10.0;
            let main = shapes::Line(start, Vec2::new((start.y + end.y) / 2.0, start.y));
            let mid = shapes::Line(
                Vec2::new((start.y + end.y) / 2.0, start.y),
                Vec2::new((start.y + end.y) / 2.0, end.y),
            );
            let main2 = shapes::Line(Vec2::new((start.y + end.y) / 2.0, end.y), end);
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
                            path: GeometryBuilder::build_as(&mid),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                    builder.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&main2),
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
        ArrowType::ParallelDoubleArrow => {
            let headlen = 10.0;
            let main = shapes::Line(start, Vec2::new((start.y + end.y) / 2.0, start.y));
            let mid = shapes::Line(
                Vec2::new((start.y + end.y) / 2.0, start.y),
                Vec2::new((start.y + end.y) / 2.0, end.y),
            );
            let main2 = shapes::Line(Vec2::new((start.y + end.y) / 2.0, end.y), end);
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
                            path: GeometryBuilder::build_as(&mid),
                            ..default()
                        },
                        Stroke::new(Color::BLACK, 2.0),
                    ));
                    builder.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&main2),
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
    }
}

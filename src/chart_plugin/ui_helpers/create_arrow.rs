use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, ShapeBundle, Stroke},
    shapes,
};

use crate::chart_plugin::ui_helpers::ArrowConnectPos;

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
            let mid_point=parallel_arrow_mid(start,end,arrow_meta);
            let main = shapes::Line(start, mid_point.0);
            let main2 = shapes::Line(mid_point.1, end);
            let mid=shapes::Line(mid_point.0,mid_point.1);
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
            let mid_point=parallel_arrow_mid(start,end,arrow_meta);
            let main = shapes::Line(start, mid_point.0);
            let main2 = shapes::Line(mid_point.1, end);
            let mid=shapes::Line(mid_point.0,mid_point.1);
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
            let mid_point=parallel_arrow_mid(start,end,arrow_meta);
            let main = shapes::Line(start, mid_point.0);
            let main2 = shapes::Line(mid_point.1, end);
            let mid=shapes::Line(mid_point.0,mid_point.1);
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
fn parallel_arrow_mid(start:Vec2,end:Vec2,arrow_meta: ArrowMeta)->(Vec2,Vec2){
    let midx=(start.x + end.x) / 2.0;
    let midy=(start.y + end.y) / 2.0;
    use ArrowConnectPos::*;
    let mid_point=match (arrow_meta.start.pos,arrow_meta.end.pos){
        (Top,Bottom)|(Bottom,Top)=>{(Vec2::new(start.x,midy),Vec2::new(end.x,midy))},
        (Left,Right)|(Right,Left)=>{(Vec2::new(midx, start.y),Vec2::new(midx, end.y))},
        //TODO 
        (Bottom,Left)|(Top,Right)|(Top,Left)|(Bottom,Right)=>{(Vec2::new(start.x,end.y),Vec2::new(start.x,end.y))},
        (Left,Bottom)|(Right,Top)|(Left,Top)|(Right,Bottom)=>{(Vec2::new(end.x,start.y),Vec2::new(end.x,start.y))},
        (_,_)=>{(Vec2::new(midx, midy),Vec2::new(midx, midy))}
    };
    return mid_point;
}

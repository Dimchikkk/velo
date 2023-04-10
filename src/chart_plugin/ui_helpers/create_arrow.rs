use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{Geometry, GeometryBuilder, ShapeBundle, Stroke},
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
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            let arrow_path = GeometryBuilder::new()
                .add(&shapes::Line(start, end))
                .add(&arrow_head(end, angle, false))
                .build();
            commands.spawn((
                ShapeBundle {
                    path: arrow_path,
                    ..default()
                },
                arrow_meta,
                Stroke::new(Color::BLACK, 2.0),
            ));
        }
        ArrowType::DoubleArrow => {
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            let arrow_path = GeometryBuilder::new()
                .add(&arrow_head(start, angle, true))
                .add(&shapes::Line(start, end))
                .add(&arrow_head(end, angle, false))
                .build();
            commands.spawn((
                ShapeBundle {
                    path: arrow_path,
                    ..default()
                },
                arrow_meta,
                Stroke::new(Color::BLACK, 2.0),
            ));
        }
        ArrowType::ParallelLine => {
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            let arrow_path = GeometryBuilder::new()
                .add(&shapes::Line(start, mid_point.0))
                .add(&shapes::Line(mid_point.0, mid_point.1))
                .add(&shapes::Line(mid_point.1, end))
                .build();
            commands.spawn((
                ShapeBundle {
                    path: arrow_path,
                    ..default()
                },
                arrow_meta,
                Stroke::new(Color::BLACK, 2.0),
            ));
        }
        ArrowType::ParallelArrow => {
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            let arrow_path = GeometryBuilder::new()
                .add(&shapes::Line(start, mid_point.0))
                .add(&shapes::Line(mid_point.0, mid_point.1))
                .add(&shapes::Line(mid_point.1, end))
                .add(&arrow_head(end, angle, false))
                .build();
            commands.spawn((
                ShapeBundle {
                    path: arrow_path,
                    ..default()
                },
                arrow_meta,
                Stroke::new(Color::BLACK, 2.0),
            ));
        }
        ArrowType::ParallelDoubleArrow => {
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            let arrow_path = GeometryBuilder::new()
                .add(&arrow_head(start, angle, true))
                .add(&shapes::Line(start, mid_point.0))
                .add(&shapes::Line(mid_point.0, mid_point.1))
                .add(&shapes::Line(mid_point.1, end))
                .add(&arrow_head(end, angle, false))
                .build();
            commands.spawn((
                ShapeBundle {
                    path: arrow_path,
                    ..default()
                },
                arrow_meta,
                Stroke::new(Color::BLACK, 2.0),
            ));
        }
    }
}
fn parallel_arrow_mid(start: Vec2, end: Vec2, arrow_meta: ArrowMeta) -> (Vec2, Vec2) {
    let mid = start + end / 2.0;
    use ArrowConnectPos::*;
    match (arrow_meta.start.pos, arrow_meta.end.pos) {
        (Top, Bottom) | (Bottom, Top) => (Vec2::new(start.x, mid.y), Vec2::new(end.x, mid.y)),
        (Left, Right) | (Right, Left) => (Vec2::new(mid.x, start.y), Vec2::new(mid.x, end.y)),
        //TODO
        (Bottom, Left) | (Top, Right) | (Top, Left) | (Bottom, Right) => {
            (Vec2::new(start.x, end.y), Vec2::new(start.x, end.y))
        }
        (Left, Bottom) | (Right, Top) | (Left, Top) | (Right, Bottom) => {
            (Vec2::new(end.x, start.y), Vec2::new(end.x, start.y))
        }
        (_, _) => (mid, mid),
    }
}
fn arrow_head(point: Vec2, angle: f32, tail: bool) -> shapes::Polygon {
    let headlen: f32 = if tail { -10.0 } else { 10.0 };
    let points = vec![
        point + Vec2::from_angle(angle - PI / 6.) * headlen,
        point,
        point + Vec2::from_angle(angle + PI / 6.) * headlen,
    ];
    shapes::Polygon {
        points,
        closed: false,
    }
}

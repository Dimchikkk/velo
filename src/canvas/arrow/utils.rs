use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, Path, ShapeBundle, Stroke},
    shapes,
};

use crate::themes::Theme;

use super::components::{ArrowConnectPos, ArrowMeta, ArrowType};

pub fn create_arrow(
    commands: &mut Commands,
    theme: &Res<Theme>,
    start: Vec2,
    end: Vec2,
    z: f32,
    arrow_meta: ArrowMeta,
) {
    let arrow_path = build_arrow(start, end, arrow_meta);
    let visibility = if arrow_meta.visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    commands.spawn((
        ShapeBundle {
            visibility,
            transform: Transform::from_xyz(0.0, 0.0, z),
            path: arrow_path,
            ..default()
        },
        arrow_meta,
        Stroke::new(theme.arrow, 1.5),
    ));
}
fn parallel_arrow_mid(start: Vec2, end: Vec2, arrow_meta: ArrowMeta) -> (Vec2, Vec2) {
    let mid = (start + end) / 2.0;
    use ArrowConnectPos::*;
    match (arrow_meta.start.pos, arrow_meta.end.pos) {
        (Top, Bottom) | (Bottom, Top) => (Vec2::new(start.x, mid.y), Vec2::new(end.x, mid.y)),
        (Left, Right) | (Right, Left) => (Vec2::new(mid.x, start.y), Vec2::new(mid.x, end.y)),
        (Bottom, Left) | (Top, Right) | (Top, Left) | (Bottom, Right) => {
            (Vec2::new(start.x, end.y), Vec2::new(start.x, end.y))
        }
        (Left, Bottom) | (Right, Top) | (Left, Top) | (Right, Bottom) => {
            (Vec2::new(end.x, start.y), Vec2::new(end.x, start.y))
        }
        (_, _) => (mid, mid),
    }
}
fn arrow_head(point: Vec2, pos: ArrowConnectPos) -> shapes::Polygon {
    let headlen: f32 = 10.0;
    use ArrowConnectPos::*;
    let angle = match pos {
        Top => PI / 2.,
        Bottom => -PI / 2.,
        Right => 0.,
        Left => PI,
    };
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
pub fn build_arrow(start: Vec2, end: Vec2, arrow_meta: ArrowMeta) -> Path {
    match arrow_meta.arrow_type {
        ArrowType::Line => {
            let main = shapes::Line(start, end);
            GeometryBuilder::build_as(&main)
        }
        ArrowType::Arrow => {
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            let headlen = 10.0;
            GeometryBuilder::new()
                .add(&shapes::Line(start, end))
                .add(&shapes::Line(
                    end,
                    end - headlen * Vec2::from_angle(angle + PI / 6.),
                ))
                .add(&shapes::Line(
                    end,
                    end - headlen * Vec2::from_angle(angle - PI / 6.),
                ))
                .build()
        }
        ArrowType::DoubleArrow => {
            let headlen = 10.0;
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            GeometryBuilder::new()
                .add(&shapes::Line(
                    start,
                    start + headlen * Vec2::from_angle(angle + PI / 6.),
                ))
                .add(&shapes::Line(
                    start,
                    start + headlen * Vec2::from_angle(angle - PI / 6.),
                ))
                .add(&shapes::Line(start, end))
                .add(&shapes::Line(
                    end,
                    end - headlen * Vec2::from_angle(angle + PI / 6.),
                ))
                .add(&shapes::Line(
                    end,
                    end - headlen * Vec2::from_angle(angle - PI / 6.),
                ))
                .build()
        }
        ArrowType::ParallelLine => {
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            GeometryBuilder::new()
                .add(&shapes::Line(start, mid_point.0))
                .add(&shapes::Line(mid_point.0, mid_point.1))
                .add(&shapes::Line(mid_point.1, end))
                .build()
        }
        ArrowType::ParallelArrow => {
            let head_pos = arrow_meta.end.pos;
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            GeometryBuilder::new()
                .add(&shapes::Line(start, mid_point.0))
                .add(&shapes::Line(mid_point.0, mid_point.1))
                .add(&shapes::Line(mid_point.1, end))
                .add(&arrow_head(end, head_pos))
                .build()
        }
        ArrowType::ParallelDoubleArrow => {
            let head_pos = arrow_meta.end.pos;
            let tail_pos = arrow_meta.start.pos;
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            GeometryBuilder::new()
                .add(&arrow_head(start, tail_pos))
                .add(&shapes::Line(start, mid_point.0))
                .add(&shapes::Line(mid_point.0, mid_point.1))
                .add(&shapes::Line(mid_point.1, end))
                .add(&arrow_head(end, head_pos))
                .build()
        }
    }
}

use std::f32::consts::PI;

use bevy::prelude::*;
//use bevy_prototype_lyon::{
//    prelude::{GeometryBuilder, Path, ShapeBundle, Stroke},
//    shapes,
//};

// use crate::chart_plugin::ui_helpers::ArrowConnectPos;

use super::components::{ArrowConnectPos, ArrowMeta, ArrowType};
pub fn create_arrow(gizmos: &mut Gizmos, start: Vec2, end: Vec2, arrow_meta: ArrowMeta) {
    // let arrow_path = build_arrow(start, end, arrow_meta);
    // commands.spawn((
    //     ShapeBundle {
    //         path: arrow_path,
    //         ..default()
    //     },
    //     arrow_meta,
    //     Stroke::new(Color::rgb(63.0 / 255.0, 81.0 / 255.0, 181.0 / 255.0), 1.5),
    // ));
    build_arrow(gizmos, start, end, arrow_meta);
}
// pub fn create_arrow2(mut gizmos:Gizmos,start:Vec2,end:Vec2,arrow_meta:ArrowMeta){
//
// }
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
// fn arrow_head(point: Vec2, pos: ArrowConnectPos) -> shapes::Polygon {
//     let headlen: f32 = 10.0;
//     use ArrowConnectPos::*;
//     let angle = match pos {
//         Top => PI / 2.,
//         Bottom => -PI / 2.,
//         Right => 0.,
//         Left => PI,
//     };
//     let points = vec![
//         point + Vec2::from_angle(angle - PI / 6.) * headlen,
//         point,
//         point + Vec2::from_angle(angle + PI / 6.) * headlen,
//     ];
//     shapes::Polygon {
//         points,
//         closed: false,
//     }
// }

pub fn build_arrow(gizmos: &mut Gizmos, start: Vec2, end: Vec2, arrow_meta: ArrowMeta) {
    match arrow_meta.arrow_type {
        ArrowType::Line => {
            //let main = shapes::Line(start, end);
            //GeometryBuilder::build_as(&main)
            gizmos.line_2d(start, end, Color::RED);
        }
        ArrowType::Arrow => {
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            let headlen = 10.0;

            gizmos.line_2d(start, end, Color::RED);
            gizmos.line_2d(
                end,
                end - headlen * Vec2::from_angle(angle + PI / 6.),
                Color::RED,
            );
            gizmos.line_2d(
                end,
                end - headlen * Vec2::from_angle(angle - PI / 6.),
                Color::RED,
            );
            // GeometryBuilder::new()
            //     .add(&shapes::Line(start, end))
            //     .add(&shapes::Line(
            //         end,
            //         end - headlen * Vec2::from_angle(angle + PI / 6.),
            //     ))
            //     .add(&shapes::Line(
            //         end,
            //         end - headlen * Vec2::from_angle(angle - PI / 6.),
            //     ))
            //     .build()
        }
        ArrowType::DoubleArrow => {
            let headlen = 10.0;
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            gizmos.line_2d(start, end, Color::RED);
            gizmos.line_2d(
                end,
                end - headlen * Vec2::from_angle(angle + PI / 6.),
                Color::RED,
            );
            gizmos.line_2d(
                end,
                end - headlen * Vec2::from_angle(angle - PI / 6.),
                Color::RED,
            );

            gizmos.line_2d(
                start,
                start - headlen * Vec2::from_angle(angle + PI / 6.),
                Color::RED,
            );
            gizmos.line_2d(
                start,
                start - headlen * Vec2::from_angle(angle - PI / 6.),
                Color::RED,
            );
            // GeometryBuilder::new()
            //     .add(&shapes::Line(
            //         start,
            //         start + headlen * Vec2::from_angle(angle + PI / 6.),
            //     ))
            //     .add(&shapes::Line(
            //         start,
            //         start + headlen * Vec2::from_angle(angle - PI / 6.),
            //     ))
            //     .add(&shapes::Line(start, end))
            //     .add(&shapes::Line(
            //         end,
            //         end - headlen * Vec2::from_angle(angle + PI / 6.),
            //     ))
            //     .add(&shapes::Line(
            //         end,
            //         end - headlen * Vec2::from_angle(angle - PI / 6.),
            //     ))
            //     .build()
        }
        ArrowType::ParallelLine => {
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            let _headlen = 10.0;
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let _angle = dy.atan2(dt);

            gizmos.line_2d(start, mid_point.0, Color::RED);
            gizmos.line_2d(mid_point.0, mid_point.1, Color::RED);
            gizmos.line_2d(mid_point.1, end, Color::RED);
            // GeometryBuilder::new()
            //     .add(&shapes::Line(start, mid_point.0))
            //     .add(&shapes::Line(mid_point.0, mid_point.1))
            //     .add(&shapes::Line(mid_point.1, end))
            //     .build()
        }
        ArrowType::ParallelArrow => {
            let _head_pos = arrow_meta.end.pos;
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            let headlen = 10.0;
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            gizmos.line_2d(start, mid_point.0, Color::RED);
            gizmos.line_2d(mid_point.0, mid_point.1, Color::RED);
            gizmos.line_2d(mid_point.1, end, Color::RED);
            gizmos.line_2d(
                end,
                end - headlen * Vec2::from_angle(angle + PI / 6.),
                Color::RED,
            );
            gizmos.line_2d(
                end,
                end - headlen * Vec2::from_angle(angle - PI / 6.),
                Color::RED,
            );
            // GeometryBuilder::new()
            //     .add(&shapes::Line(start, mid_point.0))
            //     .add(&shapes::Line(mid_point.0, mid_point.1))
            //     .add(&shapes::Line(mid_point.1, end))
            //     .add(&arrow_head(end, head_pos))
            //     .build()
        }
        ArrowType::ParallelDoubleArrow => {
            let _head_pos = arrow_meta.end.pos;
            let _tail_pos = arrow_meta.start.pos;
            let mid_point = parallel_arrow_mid(start, end, arrow_meta);
            let headlen = 10.0;
            let dt = end.x - start.x;
            let dy = end.y - start.y;
            let angle = dy.atan2(dt);
            gizmos.line_2d(
                start,
                start - headlen * Vec2::from_angle(angle + PI / 6.),
                Color::RED,
            );
            gizmos.line_2d(
                start,
                start - headlen * Vec2::from_angle(angle - PI / 6.),
                Color::RED,
            );
            gizmos.line_2d(start, mid_point.0, Color::RED);
            gizmos.line_2d(mid_point.0, mid_point.1, Color::RED);
            gizmos.line_2d(mid_point.1, end, Color::RED);
            gizmos.line_2d(
                end,
                end - headlen * Vec2::from_angle(angle + PI / 6.),
                Color::RED,
            );
            gizmos.line_2d(
                end,
                end - headlen * Vec2::from_angle(angle - PI / 6.),
                Color::RED,
            );
            // GeometryBuilder::new()
            //     .add(&arrow_head(start, tail_pos))
            //     .add(&shapes::Line(start, mid_point.0))
            //     .add(&shapes::Line(mid_point.0, mid_point.1))
            //     .add(&shapes::Line(mid_point.1, end))
            //     .add(&arrow_head(end, head_pos))
            //     .build()
        }
    }
}

pub fn get_pos(
    global_transform: &GlobalTransform,
    primary_window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let world_position = global_transform.affine().translation;
    let point = Vec2::new(world_position.x, primary_window.height() - world_position.y);
    camera.viewport_to_world_2d(camera_transform, point)
}

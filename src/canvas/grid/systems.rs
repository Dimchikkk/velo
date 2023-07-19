use crate::themes::Theme;

use super::CustomGridMaterial;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const CELL_SIZE: f32 = 12.0;

pub fn grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomGridMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    theme: Res<Theme>,
) {
    let max_size = theme.max_grid_size;
    let size = Vec2::new(max_size, max_size);
    let mesh = Mesh::from(shape::Quad { size, flip: false });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        material: materials.add(CustomGridMaterial {
            line_color: theme.canvas_bg_line_color,
            grid_size: size,
            cell_size: Vec2::splat(CELL_SIZE),
            major: 4.0,
        }),
        ..Default::default()
    });
}

pub fn update_camera_translation(
    mut query: Query<&mut Transform, (Changed<Transform>, With<OrthographicProjection>)>,
) {
    for mut transform in query.iter_mut() {
        transform.translation = transform.translation.round();
    }
}

pub fn update_cell_size(
    camera: Query<&OrthographicProjection, Changed<OrthographicProjection>>,
    grid: Query<&Handle<CustomGridMaterial>>,
    mut materials: ResMut<Assets<CustomGridMaterial>>,
) {
    for projection in camera.iter() {
        for grid_handle in grid.iter() {
            if let Some(material) = materials.get_mut(grid_handle) {
                let current_zoom = projection.scale;
                let exponent = current_zoom.log(material.major);
                let exponent = exponent.ceil();
                let grid_scale = material.major.powf(exponent);
                material.cell_size = Vec2::splat(CELL_SIZE) * grid_scale;
            }
        }
    }
}

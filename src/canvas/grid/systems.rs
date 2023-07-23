use crate::{components::MainCamera, themes::Theme};

use super::CustomGridMaterial;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const CELL_SIZE: f32 = 12.0;

#[derive(Component)]
pub struct Grid;

pub fn grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomGridMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    theme: Res<Theme>,
) {
    let max_size = 1_000_000.;
    let size = Vec2::new(max_size, max_size);
    let mesh = Mesh::from(shape::Quad { size, flip: false });

    commands
        .spawn(MaterialMesh2dBundle {
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
        })
        .insert(Grid);
}

pub fn update_grid(
    camera: Query<
        (&Camera, &GlobalTransform, &OrthographicProjection),
        (Changed<OrthographicProjection>, With<MainCamera>),
    >,
    grid: Query<(&Handle<CustomGridMaterial>, &Mesh2dHandle)>,
    mut materials: ResMut<Assets<CustomGridMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (camera, camera_transform, projection) in camera.iter() {
        for (grid_handle, mesh_handle) in grid.iter() {
            let current_zoom = projection.scale;
            let ndc = Vec3::ONE;
            let world_coords = camera.ndc_to_world(camera_transform, ndc).unwrap();
            let corner_offset = world_coords - camera_transform.translation();
            let rect = corner_offset.truncate() * 2.0;
            let side = rect.max_element();
            let size = Vec2::splat(side) * 2.0;
            if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                *mesh = shape::Quad::new(size).into();
            }
            if let Some(material) = materials.get_mut(grid_handle) {
                let exponent = current_zoom.log(material.major);
                let exponent = exponent.ceil();
                let grid_scale = material.major.powf(exponent);
                material.cell_size = Vec2::splat(CELL_SIZE) * grid_scale;
                material.grid_size = size;
            }
        }
    }
}

pub fn grid_follows_camera(
    camera: Query<
        &GlobalTransform,
        (
            With<Camera>,
            With<MainCamera>,
            Or<(Changed<OrthographicProjection>, Changed<GlobalTransform>)>,
        ),
    >,
    mut grid: Query<(&mut Transform, &Handle<CustomGridMaterial>)>,
    materials: Res<Assets<CustomGridMaterial>>,
) {
    for (mut transform, material_handle) in grid.iter_mut() {
        if let Some(material) = materials.get(material_handle) {
            for camera in camera.iter() {
                let major_grid_translation = material.cell_size * material.major;
                let camera_major_grid_translation =
                    (camera.translation().truncate() / major_grid_translation).trunc();
                let truncated_camera_major_grid_translation =
                    camera_major_grid_translation * major_grid_translation;
                transform.translation.x = truncated_camera_major_grid_translation.x;
                transform.translation.y = truncated_camera_major_grid_translation.y;
            }
        }
    }
}

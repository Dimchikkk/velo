use crate::themes::Theme;

use super::CustomGridMaterial;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
            cell_size: Vec2::new(12.0, 12.0),
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

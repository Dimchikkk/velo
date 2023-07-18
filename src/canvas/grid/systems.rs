use super::CustomGridMaterial;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomGridMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let max_size = 1000000.;
    let size = Vec2::new(max_size, max_size);
    let mesh = Mesh::from(shape::Quad { size, flip: false });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        material: materials.add(CustomGridMaterial {
            color: Color::rgba(245. / 255., 245. / 255., 245. / 255., 1.),
            line_color: Color::rgba(97. / 255., 164. / 255., 255. / 255., 0.2),
            grid_size: size,
            cell_size: Vec2::new(10.0, 10.0),
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

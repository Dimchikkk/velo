use bevy::{
    prelude::{shape::Quad, *},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::themes::Theme;

use super::CustomShadowMaterial;

#[derive(Component)]
pub struct Shadow;

// Spawn an entity using `CustomMaterial`.
pub fn spawn_shadow(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<CustomShadowMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    theme: &Res<Theme>,
    flat_size: Vec2,
) -> Entity {
    let edge_size = 0.09 * flat_size;
    let full_size = flat_size + edge_size;
    let mesh: Mesh = Quad::new(full_size).into();
    let mesh = Mesh2dHandle(meshes.add(mesh));

    let material = materials.add(CustomShadowMaterial {
        color: theme.shadow,
        flat_size,
        edge_size,
    });
    let entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform {
                translation: Vec3::new(-3., -3., 0.0009),
                ..Default::default()
            },
            ..default()
        })
        .id();
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(flat_size),
                ..default()
            },
            ..default()
        })
        .insert(Shadow)
        .add_child(entity)
        .id()
}

pub fn synchronise_shadow_sizes(
    objects: Query<&Sprite, (Changed<Sprite>, With<Shadow>)>,
    shadows: Query<(&Parent, &Handle<CustomShadowMaterial>, &Mesh2dHandle)>,
    mut materials: ResMut<Assets<CustomShadowMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (parent, mat, mesh) in shadows.iter() {
        let Ok(sprite) = objects.get(parent.get()) else {
            continue;
        };
        let Some(material) = materials.get_mut(mat) else {
            continue;
        };
        let Some(mesh) = meshes.get_mut(&mesh.0) else {
            continue;
        };

        let flat_size = sprite.custom_size.unwrap();
        let edge_size = material.edge_size;
        let full_size = flat_size + edge_size;
        *mesh = Quad::new(full_size).into();
        material.flat_size = flat_size;
    }
}

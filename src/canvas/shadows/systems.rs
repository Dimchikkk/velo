use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_cosmic_edit::CosmicEdit;

use crate::{
    themes::Theme,
    ui_plugin::ui_helpers::{RawText, VeloNode, VeloShape},
};

use super::CustomShadowMaterial;

fn unnormalize_uv(uv: f32, size: f32) -> f32 {
    size * (uv - 0.5)
}

#[derive(Component)]
pub struct Shadow;

// Spawn an entity using `CustomMaterial`.
pub fn spawn_shadow(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<CustomShadowMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    theme: &Res<Theme>,
) -> Entity {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<[f32; 2]>::new());
    mesh.set_indices(Some(Indices::U32(Vec::new())));

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(CustomShadowMaterial {
                color: theme.shadow,
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.09),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Shadow)
        .id()
}

pub fn update_shadows(
    mut meshes: ResMut<Assets<Mesh>>,
    velo_node_query: Query<(&GlobalTransform, &Children), With<VeloNode>>,
    velo_shape_query: Query<&Children, With<VeloShape>>,
    shadows_query: Query<&Mesh2dHandle, With<Shadow>>,
    cosmic_query: Query<&CosmicEdit, With<RawText>>,
) {
    for (global_transform, children) in velo_node_query.iter() {
        let mut velo_shape_children = None;
        let mut mesh_handle = None;

        for child in children {
            if let Ok(velo_shape_component) = velo_shape_query.get(*child) {
                velo_shape_children = Some(velo_shape_component);
            }
            if let Ok(mesh_handle_component) = shadows_query.get(*child) {
                mesh_handle = Some(mesh_handle_component);
            }
        }

        if mesh_handle.is_none() {
            continue;
        }

        let mesh_handle = mesh_handle.unwrap();
        let velo_shape_children = velo_shape_children.unwrap();
        for velo_shape_child in velo_shape_children.iter() {
            if let Ok(cosmic_edit) = cosmic_query.get(*velo_shape_child) {
                let mesh = meshes.get_mut(&mesh_handle.0).unwrap();
                let translation = global_transform.translation();
                let width = cosmic_edit.width;
                let height = cosmic_edit.height;

                let half_width = 1.25 * width / 2.0;
                let half_height = 1.25 * height / 2.0;

                let offset_x = 0.04 * width;
                let offset_y = 0.04 * height;

                let vertices: Vec<[f32; 3]> = [
                    [
                        translation.x - half_width - offset_x,
                        translation.y - half_height - offset_y,
                        0.0,
                    ],
                    [
                        translation.x + half_width - offset_x,
                        translation.y - half_height - offset_y,
                        0.0,
                    ],
                    [
                        translation.x + half_width - offset_x,
                        translation.y + half_height - offset_y,
                        0.0,
                    ],
                    [
                        translation.x - half_width - offset_x,
                        translation.y + half_height - offset_y,
                        0.0,
                    ],
                ]
                .iter()
                .map(|&v| [v[0], v[1], v[2]])
                .collect();

                let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];

                let uvs: Vec<[f32; 2]> = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]
                    .iter()
                    .map(|uv| [unnormalize_uv(uv[0], width), unnormalize_uv(uv[1], height)])
                    .collect();

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                mesh.set_indices(Some(Indices::U32(indices)));
            }
        }
    }
}

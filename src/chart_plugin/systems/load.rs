use base64::{engine::general_purpose, Engine};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use image::{load_from_memory_with_format, ImageFormat};
use serde_json::Value;

use crate::{AppState, JsonNode, LoadRequest};

use super::ui_helpers::{spawn_node, ArrowMeta, CreateArrow, NodeMeta, Rectangle, ReflectableUuid};

pub fn should_load(request: Option<Res<LoadRequest>>) -> bool {
    request.is_some()
}

pub fn remove_load_request(world: &mut World) {
    world.remove_resource::<LoadRequest>().unwrap();
}

pub fn load_json(
    old_nodes: Query<Entity, With<Rectangle>>,
    old_arrows: Query<Entity, With<ArrowMeta>>,
    request: Res<LoadRequest>,
    mut state: ResMut<AppState>,
    mut commands: Commands,
    mut res_images: ResMut<Assets<Image>>,
    mut create_arrow: EventWriter<CreateArrow>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");

    for entity in old_arrows.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in old_nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let mut json: Value = match &request.path {
        Some(path) => {
            let json = std::fs::read_to_string(path).expect("Error reading state from file");
            serde_json::from_str(&json).unwrap()
        }
        None => {
            let json = if state.checkpoints.len() == 1 {
                state.checkpoints.back().unwrap().clone()
            } else {
                state.checkpoints.pop_back().unwrap()
            };
            serde_json::from_str(&json).unwrap()
        }
    };
    let images = json["images"].as_object().unwrap();
    let nodes = json["nodes"].as_array().unwrap();
    for node in nodes.iter() {
        let json_node: JsonNode = serde_json::from_value(node.clone()).unwrap();
        let image: Option<UiImage> = match images.get(&json_node.id.to_string()) {
            Some(image) => {
                let image_bytes = general_purpose::STANDARD
                    .decode(image.as_str().unwrap().as_bytes())
                    .unwrap();
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let img = load_from_memory_with_format(&image_bytes, ImageFormat::Png).unwrap();
                    let size: Extent3d = Extent3d {
                        width: img.width(),
                        height: img.height(),
                        ..Default::default()
                    };
                    let image = Image::new(
                        size,
                        TextureDimension::D2,
                        img.into_bytes(),
                        TextureFormat::Rgba8UnormSrgb,
                    );
                    let image_handle = res_images.add(image);
                    Some(image_handle.into())
                }
            }
            None => None,
        };
        state.entity_to_edit = Some(ReflectableUuid(json_node.id));
        // ideally AddRect event should be fired instead of calling spawn_node directly
        let entity = spawn_node(
            &mut commands,
            NodeMeta {
                font: font.clone(),
                size: (json_node.width, json_node.height),
                id: ReflectableUuid(json_node.id),
                image: image.clone(),
                text: json_node.text.clone(),
                bg_color: json_node.bg_color,
                position: (json_node.left, json_node.bottom),
                tags: json_node.tags,
                text_pos: json_node.text_pos
            },
        );
        commands.entity(state.main_panel.unwrap()).add_child(entity);
    }

    let arrows = json["arrows"].as_array_mut().unwrap();
    for arrow in arrows.iter() {
        let arrow_meta: ArrowMeta = serde_json::from_value(arrow.clone()).unwrap();
        create_arrow.send(CreateArrow {
            start: arrow_meta.start,
            end: arrow_meta.end,
        });
    }
}

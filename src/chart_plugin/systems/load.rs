use std::collections::HashMap;

use base64::{engine::general_purpose, Engine};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use bevy_pkv::PkvStore;
use image::{load_from_memory_with_format, ImageFormat};
use serde_json::Value;

use crate::{AppState, Doc, JsonNode, LoadRequest, MAX_DOCS_IN_MEMORY};

use super::ui_helpers::{
    add_tab, spawn_node, ArrowMeta, BottomPanel, CreateArrow, NodeMeta, Rectangle, ReflectableUuid,
    SelectedTab,
};

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
    mut selected_tabs_query: Query<Entity, With<SelectedTab>>,
    mut bottom_panel: Query<Entity, With<BottomPanel>>,
    pkv: ResMut<PkvStore>,
) {
    state.entity_to_edit = None;
    state.tab_to_edit = None;
    state.doc_to_edit = None;
    state.hold_entity = None;
    state.entity_to_resize = None;
    state.arrow_to_draw_start = None;

    let font = asset_server.load("fonts/iosevka-regular.ttf");

    for entity in old_arrows.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in old_nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in selected_tabs_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    if let Some(doc_id) = &request.doc_id {
        if state.docs.contains_key(doc_id) {
            state.current_document = Some(*doc_id);
        } else if let Ok(docs) = pkv.get::<HashMap<ReflectableUuid, Doc>>("docs") {
            if docs.contains_key(doc_id) {
                if (state.docs.len() as i32) >= MAX_DOCS_IN_MEMORY {
                    let keys = state.docs.keys().cloned().collect::<Vec<_>>();
                    state.docs.remove(&keys[0]);
                }
                state
                    .docs
                    .insert(*doc_id, docs.get(doc_id).unwrap().clone());
                state.current_document = Some(*doc_id);
            }
        }
    }

    let bottom_panel = bottom_panel.single_mut();
    let doc = if request.doc_id.is_some() {
        request.doc_id.unwrap()
    } else {
        state.current_document.unwrap()
    };
    for tab in state.docs.get_mut(&doc).unwrap().tabs.iter() {
        let tab_view = add_tab(&mut commands, font.clone(), tab.name.clone(), tab.id);
        commands.entity(bottom_panel).add_child(tab_view);
    }

    for tab in state.docs.get_mut(&doc).unwrap().tabs.iter_mut() {
        if tab.is_active {
            if tab.checkpoints.is_empty() {
                break;
            }

            let json = if request.drop_last_checkpoint && tab.checkpoints.len() > 1 {
                tab.checkpoints.pop_back().unwrap()
            } else {
                tab.checkpoints.back().unwrap().clone()
            };
            let mut json: Value = serde_json::from_str(&json).unwrap();
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
                            let img = load_from_memory_with_format(&image_bytes, ImageFormat::Png)
                                .unwrap();
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
                        #[cfg(target_arch = "wasm32")]
                        None
                    }
                    None => None,
                };
                // ideally AddRect event should be fired instead of calling spawn_node directly
                let entity = spawn_node(
                    &mut commands,
                    NodeMeta {
                        font: font.clone(),
                        size: (json_node.width, json_node.height),
                        id: ReflectableUuid(json_node.id),
                        image: image.clone(),
                        text: json_node.text.text.clone(),
                        bg_color: json_node.bg_color,
                        position: (json_node.left, json_node.bottom),
                        tags: json_node.tags,
                        text_pos: json_node.text.pos,
                        z_index: json_node.z_index,
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
                    arrow_type: arrow_meta.arrow_type,
                });
            }
            break;
        }
    }
}

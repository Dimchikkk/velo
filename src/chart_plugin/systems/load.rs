use std::collections::HashMap;

use base64::{engine::general_purpose, Engine};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use super::{
    ui_helpers::{add_tab, spawn_node, BottomPanel, NodeMeta, TabContainer},
    HighlightEvent, VeloNodeContainer, MainPanel,
};
use crate::canvas::arrow::components::ArrowMeta;
use crate::canvas::arrow::events::CreateArrow;
use crate::components::Doc;
use crate::resources::{AppState, LoadRequest};
use crate::utils::ReflectableUuid;
use crate::{JsonNode, UiState, MAX_SAVED_DOCS_IN_MEMORY};
use bevy_pkv::PkvStore;
#[cfg(not(target_arch = "wasm32"))]
use image::{load_from_memory_with_format, ImageFormat};
use serde_json::Value;

pub fn should_load(request: Option<Res<LoadRequest>>) -> bool {
    request.is_some()
}

pub fn remove_load_request(world: &mut World) {
    world.remove_resource::<LoadRequest>().unwrap();
}

pub fn load_json(
    old_nodes: Query<Entity, With<VeloNodeContainer>>,
    mut old_arrows: Query<(Entity, &mut Visibility), With<ArrowMeta>>,
    request: Res<LoadRequest>,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    mut res_images: ResMut<Assets<Image>>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut selected_tabs_query: Query<Entity, With<TabContainer>>,
    mut bottom_panel: Query<Entity, With<BottomPanel>>,
    pkv: ResMut<PkvStore>,
    mut events: EventWriter<HighlightEvent>,
    main_panel_query: Query<Entity, With<MainPanel>>,
) {
    *ui_state = UiState::default();

    let bottom_panel = bottom_panel.single_mut();

    #[allow(unused)]
    for (entity, mut visibility) in &mut old_arrows.iter_mut() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            commands.entity(entity).despawn_recursive();
        }
        #[cfg(target_arch = "wasm32")]
        {
            *visibility = Visibility::Hidden;
        }
    }
    for entity in old_nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in selected_tabs_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    let doc_id = if request.doc_id.is_some() {
        request.doc_id.unwrap()
    } else {
        app_state.current_document.unwrap()
    };

    if app_state.docs.contains_key(&doc_id) {
        app_state.current_document = Some(doc_id);
    } else if let Ok(docs) = pkv.get::<HashMap<ReflectableUuid, Doc>>("docs") {
        if docs.contains_key(&doc_id) {
            while (app_state.docs.len() as i32) >= MAX_SAVED_DOCS_IN_MEMORY {
                let keys = app_state.docs.keys().cloned().collect::<Vec<_>>();
                app_state.docs.remove(&keys[0]);
            }
            app_state
                .docs
                .insert(doc_id, docs.get(&doc_id).unwrap().clone());
            app_state.current_document = Some(doc_id);
        } else {
            panic!("Document not found in pkv");
        }
    }
    let doc_id = app_state.current_document.unwrap();

    let mut tabs = vec![];
    for tab in app_state.docs.get_mut(&doc_id).unwrap().tabs.iter() {
        let tab_view = add_tab(&mut commands, tab.name.clone(), tab.id);
        tabs.push(tab_view);
    }
    commands.entity(bottom_panel).insert_children(0, &tabs);

    for tab in app_state.docs.get_mut(&doc_id).unwrap().tabs.iter_mut() {
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
                commands
                    .entity(main_panel_query.single())
                    .add_child(entity);
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
    events.send(HighlightEvent);
}

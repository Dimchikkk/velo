use base64::{engine::general_purpose, Engine};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use super::{
    load_doc_to_memory,
    ui_helpers::{add_tab, spawn_node, BottomPanel, NodeMeta, TabContainer},
    DeleteDoc, DeleteTab, MainPanel, VeloNodeContainer,
};
use crate::canvas::arrow::events::CreateArrowEvent;
use crate::{canvas::arrow::components::ArrowMeta, resources::LoadTabRequest};

use crate::resources::{AppState, LoadDocRequest};
use crate::utils::ReflectableUuid;
use crate::{JsonNode, UiState};
use bevy_pkv::PkvStore;
use image::{load_from_memory_with_format, ImageFormat};
use serde_json::Value;

pub fn should_load_doc(request: Option<Res<LoadDocRequest>>) -> bool {
    request.is_some()
}

pub fn should_load_tab(request: Option<Res<LoadTabRequest>>) -> bool {
    request.is_some()
}

pub fn remove_load_tab_request(world: &mut World) {
    world.remove_resource::<LoadTabRequest>().unwrap();
}

pub fn remove_load_doc_request(world: &mut World) {
    world.remove_resource::<LoadDocRequest>().unwrap();
}

pub fn load_doc(
    request: Res<LoadDocRequest>,
    mut app_state: ResMut<AppState>,
    mut commands: Commands,
    mut bottom_panel: Query<Entity, With<BottomPanel>>,
    mut pkv: ResMut<PkvStore>,
    asset_server: Res<AssetServer>,
    mut tabs_query: Query<Entity, With<TabContainer>>,
    mut delete_doc: Query<(&mut Visibility, &DeleteDoc), With<DeleteDoc>>,
) {
    let bottom_panel = bottom_panel.single_mut();
    let doc_id = request.doc_id;
    for (mut visibility, doc) in delete_doc.iter_mut() {
        if doc.id == doc_id {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    load_doc_to_memory(doc_id, &mut app_state, &mut pkv);

    let mut tabs = vec![];
    for entity in tabs_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for tab in app_state.docs.get_mut(&doc_id).unwrap().tabs.iter() {
        let tab_view: Entity = add_tab(
            &mut commands,
            &asset_server,
            tab.name.clone(),
            tab.id,
            tab.is_active,
        );
        tabs.push(tab_view);
        if tab.is_active {
            commands.insert_resource(LoadTabRequest {
                doc_id,
                tab_id: tab.id,
                drop_last_checkpoint: false,
            });
        }
    }
    commands.entity(bottom_panel).insert_children(0, &tabs);
}

pub fn load_tab(
    asset_server: Res<AssetServer>,
    old_nodes: Query<Entity, With<VeloNodeContainer>>,
    mut old_arrows: Query<(Entity, &mut Visibility), With<ArrowMeta>>,
    request: Res<LoadTabRequest>,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    mut res_images: ResMut<Assets<Image>>,
    mut create_arrow: EventWriter<CreateArrowEvent>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    mut delete_tab: Query<(&mut Visibility, &DeleteTab), (With<DeleteTab>, Without<ArrowMeta>)>,
) {
    *ui_state = UiState::default();

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

    let doc_id = request.doc_id;
    for (mut visibility, tab) in delete_tab.iter_mut() {
        if tab.id == request.tab_id {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    for tab in app_state.docs.get_mut(&doc_id).unwrap().tabs.iter_mut() {
        if tab.id == request.tab_id {
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
                        let img =
                            load_from_memory_with_format(&image_bytes, ImageFormat::Png).unwrap();
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
                    None => None,
                };
                // ideally AddRect event should be fired instead of calling spawn_node directly
                let entity = spawn_node(
                    &mut commands,
                    &asset_server,
                    NodeMeta {
                        size: (json_node.width, json_node.height),
                        id: ReflectableUuid(json_node.id),
                        image: image.clone(),
                        text: json_node.text.text.clone(),
                        bg_color: json_node.bg_color,
                        position: (json_node.left, json_node.bottom),
                        text_pos: json_node.text.pos,
                        z_index: json_node.z_index,
                        is_active: false,
                    },
                );
                commands.entity(main_panel_query.single()).add_child(entity);
            }

            let arrows = json["arrows"].as_array_mut().unwrap();
            for arrow in arrows.iter() {
                let arrow_meta: ArrowMeta = serde_json::from_value(arrow.clone()).unwrap();
                create_arrow.send(CreateArrowEvent {
                    start: arrow_meta.start,
                    end: arrow_meta.end,
                    arrow_type: arrow_meta.arrow_type,
                });
            }
            break;
        }
    }
}

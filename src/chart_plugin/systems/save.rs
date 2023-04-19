use base64::{engine::general_purpose, Engine};
use bevy::prelude::*;

use bevy_pkv::PkvStore;
#[cfg(not(target_arch = "wasm32"))]
use image::*;

use serde_json::json;
use std::{collections::HashMap, io::Cursor};

use super::ui_helpers::{EditableText, VeloNode};
use super::VeloNodeContainer;
use crate::canvas::arrow::components::ArrowMeta;
use crate::components::Doc;
use crate::resources::AppState;
use crate::resources::SaveRequest;
use crate::utils::ReflectableUuid;
use crate::{
    chart_plugin::ui_helpers::style_to_pos, JsonNode, JsonNodeText, MAX_CHECKPOINTS,
    MAX_SAVED_DOCS_IN_MEMORY,
};

pub fn should_save(request: Option<Res<SaveRequest>>) -> bool {
    request.is_some()
}

pub fn remove_save_request(world: &mut World) {
    world.remove_resource::<SaveRequest>().unwrap();
}

pub fn save_json(
    images: Res<Assets<Image>>,
    rec_container_query: Query<&Style, With<VeloNodeContainer>>,
    rec_query: Query<
        (
            &VeloNode,
            &UiImage,
            &BackgroundColor,
            &Children,
            &ZIndex,
            &Parent,
        ),
        With<VeloNode>,
    >,
    arrows: Query<(&ArrowMeta, &Visibility), With<ArrowMeta>>,
    request: Res<SaveRequest>,
    mut app_state: ResMut<AppState>,
    text_query: Query<&mut Text, With<EditableText>>,
    mut pkv: ResMut<PkvStore>,
) {
    let mut json = json!({
        "images": {},
        "nodes": [],
        "arrows": [],
    });
    let json_images = json["images"].as_object_mut().unwrap();
    for (rect, image, _, _, _, _) in rec_query.iter() {
        if let Some(image) = images.get(&image.texture) {
            if let Ok(img) = image.clone().try_into_dynamic() {
                let mut image_data: Vec<u8> = Vec::new();
                #[cfg(not(target_arch = "wasm32"))]
                img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
                    .unwrap();
                let res_base64 = general_purpose::STANDARD.encode(image_data);
                json_images.insert(rect.id.0.to_string(), json!(res_base64));
            }
        }
    }

    let json_nodes = json["nodes"].as_array_mut().unwrap();
    for (rect, _, bg_color, children, z_index, parent) in rec_query.iter() {
        let text = text_query.get(children[children.len() - 1]).unwrap();
        let text = text.sections[0].value.clone();
        let style = rec_container_query.get(parent.get()).unwrap();
        let left = style.position.left;
        let bottom = style.position.bottom;
        let size = style.size;
        let bg_color = bg_color.0;
        let z_index = match *z_index {
            ZIndex::Local(v) => v,
            _ => -1,
        };
        json_nodes.push(json!(JsonNode {
            node_type: crate::NodeType::Rect,
            id: rect.id.0,
            left,
            bottom,
            width: size.width,
            height: size.height,
            bg_color,
            text: JsonNodeText {
                text,
                pos: style_to_pos((style.justify_content, style.align_items)),
            },
            z_index,
            tags: vec![],
        }));
    }

    let json_arrows = json["arrows"].as_array_mut().unwrap();
    for (arrow_meta, visibility) in arrows.iter() {
        if visibility != Visibility::Hidden {
            json_arrows.push(json!(arrow_meta));
        }
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
    let doc_id: ReflectableUuid = app_state.current_document.unwrap();

    for tab in &mut app_state.docs.get_mut(&doc_id).unwrap().tabs {
        if request.tab_id.is_some() {
            if tab.id == request.tab_id.unwrap() {
                if (tab.checkpoints.len() as i32) > MAX_CHECKPOINTS {
                    tab.checkpoints.pop_front();
                }
                tab.checkpoints.push_back(json.to_string());
                break;
            }
        } else if tab.is_active {
            if (tab.checkpoints.len() as i32) > MAX_CHECKPOINTS {
                tab.checkpoints.pop_front();
            }
            tab.checkpoints.push_back(json.to_string());
            break;
        }
    }

    if let Some(doc_id) = request.doc_id {
        if let Ok(mut docs) = pkv.get::<HashMap<ReflectableUuid, Doc>>("docs") {
            docs.insert(doc_id, app_state.docs.get(&doc_id).unwrap().clone());
            pkv.set("docs", &docs).unwrap();
        } else {
            let mut docs = HashMap::new();
            docs.insert(doc_id, app_state.docs.get(&doc_id).unwrap().clone());
            pkv.set("docs", &docs).unwrap();
        }
        if let Ok(mut tags) = pkv.get::<HashMap<ReflectableUuid, Vec<String>>>("tags") {
            let doc = app_state.docs.get(&doc_id).unwrap();
            let tags = tags.get_mut(&doc_id).unwrap();
            tags.append(&mut doc.tags.clone());
            pkv.set("tags", &tags).unwrap();
        } else {
            let doc = app_state.docs.get(&doc_id).unwrap();
            pkv.set("tags", &doc.tags).unwrap();
        }
        if let Ok(mut names) = pkv.get::<HashMap<ReflectableUuid, String>>("names") {
            let doc = app_state.docs.get(&doc_id).unwrap();
            names.insert(doc.id, doc.name.clone());
            pkv.set("names", &names).unwrap();
        } else {
            let doc = app_state.docs.get(&doc_id).unwrap();
            let mut names = HashMap::new();
            names.insert(doc.id, doc.name.clone());
            pkv.set("names", &names).unwrap();
        }
        pkv.set("last_saved", &doc_id).unwrap();
    }
}

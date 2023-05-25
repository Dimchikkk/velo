use base64::{engine::general_purpose, Engine};
use bevy::prelude::*;

use bevy_cosmic_edit::{get_cosmic_text, CosmicEditImage};
use bevy_pkv::PkvStore;
use image::*;

use serde_json::json;
use std::{collections::HashMap, io::Cursor};

use super::ui_helpers::VeloNode;
use super::{RawText, SaveStoreEvent, VeloNodeContainer};
use crate::canvas::arrow::components::ArrowMeta;
use crate::components::Doc;
use crate::resources::SaveDocRequest;
use crate::resources::{AppState, SaveTabRequest};
use crate::utils::{load_doc_to_memory, ReflectableUuid};
use crate::{ui_plugin::ui_helpers::style_to_pos, JsonNode, JsonNodeText, MAX_CHECKPOINTS};

pub fn should_save_doc(request: Option<Res<SaveDocRequest>>) -> bool {
    request.is_some()
}

pub fn should_save_tab(request: Option<Res<SaveTabRequest>>) -> bool {
    request.is_some()
}

pub fn remove_save_doc_request(world: &mut World) {
    world.remove_resource::<SaveDocRequest>().unwrap();
}

pub fn remove_save_tab_request(world: &mut World) {
    world.remove_resource::<SaveTabRequest>().unwrap();
}

pub fn save_doc(
    request: Res<SaveDocRequest>,
    mut app_state: ResMut<AppState>,
    mut pkv: ResMut<PkvStore>,
    mut commands: Commands,
    mut events: EventWriter<SaveStoreEvent>,
) {
    let doc_id = request.doc_id;

    load_doc_to_memory(doc_id, &mut app_state, &mut pkv);

    for tab in app_state.docs.get_mut(&doc_id).unwrap().tabs.iter() {
        if tab.is_active {
            commands.insert_resource(SaveTabRequest {
                doc_id,
                tab_id: tab.id,
            });
        }
    }
    // event is used for running save_tab logic before saving to store
    events.send(SaveStoreEvent {
        doc_id,
        path: request.path.clone(),
    });
}

pub fn save_to_store(
    mut pkv: ResMut<PkvStore>,
    mut app_state: ResMut<AppState>,
    mut events: EventReader<SaveStoreEvent>,
) {
    for event in events.iter() {
        let doc_id = event.doc_id;
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
            if let Some(tags) = tags.get_mut(&doc_id) {
                tags.append(&mut doc.tags.clone());
            } else {
                tags.insert(doc.id, doc.tags.clone());
            }
            pkv.set("tags", &tags).unwrap();
        } else {
            let doc = app_state.docs.get(&doc_id).unwrap();
            let mut tags = HashMap::new();
            tags.insert(doc.id, doc.tags.clone());
            pkv.set("tags", &tags).unwrap();
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

        if let Some(path) = event.path.clone() {
            let current_doc = app_state.docs.get(&doc_id).unwrap().clone();
            std::fs::write(path, serde_json::to_string_pretty(&current_doc).unwrap())
                .expect("Error saving current document to file")
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(index) = &mut app_state.search_index {
                let pool = bevy::tasks::IoTaskPool::get();
                let tabs_to_delete = std::sync::Arc::new(index.tabs_to_delete.clone());
                let node_updates = std::sync::Arc::new(index.node_updates.clone());
                index.tabs_to_delete.clear();
                index.node_updates.clear();
                let index = std::sync::Arc::new(index.index.clone());
                pool.spawn(async move {
                    let _ = super::clear_tabs_index(&index, &tabs_to_delete);
                    let _ = super::update_search_index(&index, &node_updates);
                })
                .detach();
            }
        }
    }
}

pub fn save_tab(
    images: Res<Assets<Image>>,
    node_container_query: Query<&Style, With<VeloNodeContainer>>,
    node_query: Query<
        (
            &VeloNode,
            &UiImage,
            &BackgroundColor,
            &ZIndex,
            &Parent,
            &Style,
        ),
        (With<VeloNode>, Without<VeloNodeContainer>),
    >,
    arrows: Query<(&ArrowMeta, &Visibility), With<ArrowMeta>>,
    request: Res<SaveTabRequest>,
    mut app_state: ResMut<AppState>,
    text_query: Query<(&RawText, &CosmicEditImage), With<RawText>>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(index) = &mut app_state.search_index {
        index.tabs_to_delete.insert(request.tab_id.0);
    }
    let mut json = json!({
        "images": {},
        "nodes": [],
        "arrows": [],
    });
    let json_images = json["images"].as_object_mut().unwrap();
    for (rect, image, _, _, _, _) in node_query.iter() {
        if let Some(image) = images.get(&image.texture) {
            if let Ok(img) = image.clone().try_into_dynamic() {
                let mut image_data: Vec<u8> = Vec::new();
                img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
                    .unwrap();
                let res_base64 = general_purpose::STANDARD.encode(image_data);
                json_images.insert(rect.id.0.to_string(), json!(res_base64));
            }
        }
    }

    let json_nodes = json["nodes"].as_array_mut().unwrap();
    for (node, _, bg_color, z_index, parent, test_pos_style) in node_query.iter() {
        for (editable_text, cosmic_edit) in text_query.iter() {
            if node.id == editable_text.id {
                let str = get_cosmic_text(&cosmic_edit.editor);
                let style: &Style = node_container_query.get(parent.get()).unwrap();
                let left = style.position.left;
                let bottom = style.position.bottom;
                let size = style.size;
                let bg_color = bg_color.0;
                let z_index = match *z_index {
                    ZIndex::Local(v) => v,
                    _ => -1,
                };
                json_nodes.push(json!(JsonNode {
                    node_type: node.node_type.clone(),
                    id: node.id.0,
                    left,
                    bottom,
                    width: size.width,
                    height: size.height,
                    bg_color,
                    text: JsonNodeText {
                        text: str.clone(),
                        pos: style_to_pos((
                            test_pos_style.justify_content,
                            test_pos_style.align_items
                        )),
                    },
                    z_index,
                }));
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(index) = &mut app_state.search_index {
                    index.node_updates.insert(
                        super::NodeSearchLocation {
                            doc_id: request.doc_id.0,
                            tab_id: request.tab_id.0,
                            node_id: node.id.0,
                        },
                        str.clone(),
                    );
                }
            }
        }
    }

    let json_arrows = json["arrows"].as_array_mut().unwrap();
    for (arrow_meta, visibility) in arrows.iter() {
        if visibility != Visibility::Hidden {
            json_arrows.push(json!(arrow_meta));
        }
    }

    let doc_id = request.doc_id;

    for tab in &mut app_state.docs.get_mut(&doc_id).unwrap().tabs {
        if request.tab_id == tab.id {
            if (tab.checkpoints.len() as i32) > MAX_CHECKPOINTS {
                tab.checkpoints.pop_front();
            }
            if let Some(last) = tab.checkpoints.back() {
                if last == &json.to_string() {
                    break;
                }
            }
            tab.checkpoints.push_back(json.to_string());
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    /// No PKV with tags
    fn test_save_doc1() {
        // Setup
        let mut app = App::new();
        app.add_systems((save_doc, save_to_store.after(save_doc)));
        let temp_dir = tempdir().unwrap();
        let temp_file_path = temp_dir.path().join("test_doc.json");
        let doc_id = ReflectableUuid::generate();
        let tab_id = ReflectableUuid::generate();
        let mut app_state = AppState::default();
        app_state.docs.insert(
            doc_id,
            Doc {
                id: doc_id,
                name: "test_doc".to_string(),
                tags: vec!["test_tag".to_string()],
                tabs: vec![crate::components::Tab {
                    id: tab_id,
                    is_active: true,
                    name: "Test tab".to_string(),
                    checkpoints: std::collections::VecDeque::new(),
                }],
            },
        );
        let request = SaveDocRequest {
            doc_id,
            path: Some(temp_file_path.clone()),
        };
        app.insert_resource(request);
        app.add_event::<SaveStoreEvent>();
        PkvStore::new("test", "test").clear().unwrap();
        app.insert_resource(PkvStore::new("test", "test"));
        app.insert_resource(app_state);

        // Run systems
        app.update();

        // Assertions
        let pkv = app.world.resource::<PkvStore>();
        let saved_docs: HashMap<ReflectableUuid, Doc> = pkv.get("docs").unwrap();
        assert_eq!(saved_docs.get(&doc_id).unwrap().name, "test_doc");
        assert!(saved_docs.get(&doc_id).unwrap().tabs[0].is_active);
        let saved_tags: HashMap<ReflectableUuid, Vec<String>> = pkv.get("tags").unwrap();
        assert_eq!(
            saved_tags.get(&doc_id).unwrap(),
            &vec!["test_tag".to_string()]
        );
        let saved_names: HashMap<ReflectableUuid, String> = pkv.get("names").unwrap();
        assert_eq!(saved_names.get(&doc_id).unwrap(), "test_doc");
        assert_eq!(pkv.get::<ReflectableUuid>("last_saved").unwrap(), doc_id);
        let file_contents = std::fs::read_to_string(temp_file_path).unwrap();
        let saved_doc: Doc = serde_json::from_str(&file_contents).unwrap();
        assert_eq!(saved_doc.name, "test_doc");
        assert!(saved_doc.tabs[0].is_active);
    }

    #[test]
    ///the PKV store has tags, but not for the document being saved:
    fn test_save_doc2() {
        // Setup
        let mut app = App::new();
        app.add_systems((save_doc, save_to_store.after(save_doc)));
        let temp_dir = tempdir().unwrap();
        let temp_file_path = temp_dir.path().join("test_doc.json");
        let doc_id = ReflectableUuid::generate();
        let tab_id = ReflectableUuid::generate();
        let mut app_state = AppState::default();
        app_state.docs.insert(
            doc_id,
            Doc {
                id: doc_id,
                name: "test_doc".to_string(),
                tags: vec!["test_tag_1".to_string()],
                tabs: vec![crate::components::Tab {
                    id: tab_id,
                    is_active: true,
                    name: "Test tab".to_string(),
                    checkpoints: std::collections::VecDeque::new(),
                }],
            },
        );
        let request = SaveDocRequest {
            doc_id,
            path: Some(temp_file_path.clone()),
        };
        app.insert_resource(request);
        PkvStore::new("test", "test1").clear().unwrap();
        let mut pkv = PkvStore::new("test", "test1");
        let mut tags = HashMap::new();
        tags.insert(ReflectableUuid::generate(), vec!["test_tag_2".to_string()]);
        pkv.set("tags", &tags).unwrap();
        app.add_event::<SaveStoreEvent>();
        app.insert_resource(pkv);
        app.insert_resource(app_state);

        // Run systems
        app.update();

        // Assertions
        let pkv = app.world.resource::<PkvStore>();
        let saved_docs: HashMap<ReflectableUuid, Doc> = pkv.get("docs").unwrap();
        assert_eq!(saved_docs.get(&doc_id).unwrap().name, "test_doc");
        assert!(saved_docs.get(&doc_id).unwrap().tabs[0].is_active);
        let saved_tags: HashMap<ReflectableUuid, Vec<String>> = pkv.get("tags").unwrap();
        assert_eq!(
            saved_tags.get(&doc_id).unwrap(),
            &vec!["test_tag_1".to_string()]
        );
        let saved_names: HashMap<ReflectableUuid, String> = pkv.get("names").unwrap();
        assert_eq!(saved_names.get(&doc_id).unwrap(), "test_doc");
        assert_eq!(pkv.get::<ReflectableUuid>("last_saved").unwrap(), doc_id);
        let file_contents = std::fs::read_to_string(temp_file_path).unwrap();
        let saved_doc: Doc = serde_json::from_str(&file_contents).unwrap();
        assert_eq!(saved_doc.name, "test_doc");
        assert!(saved_doc.tabs[0].is_active);
    }

    #[test]
    /// the PKV store already has tags for the document being saved.
    fn test_save_doc3() {
        // Setup
        let mut app = App::new();
        app.add_systems((save_doc, save_to_store.after(save_doc)));
        let temp_dir = tempdir().unwrap();
        let temp_file_path = temp_dir.path().join("test_doc.json");
        let doc_id = ReflectableUuid::generate();
        let tab_id = ReflectableUuid::generate();
        let mut app_state = AppState::default();
        let existing_tags = vec!["test_tag_2".to_string(), "test_tag_1".to_string()];
        app_state.docs.insert(
            doc_id,
            Doc {
                id: doc_id,
                name: "test_doc".to_string(),
                tags: vec!["test_tag_1".to_string()],
                tabs: vec![crate::components::Tab {
                    id: tab_id,
                    is_active: true,
                    name: "Test tab".to_string(),
                    checkpoints: std::collections::VecDeque::new(),
                }],
            },
        );
        let request = SaveDocRequest {
            doc_id,
            path: Some(temp_file_path.clone()),
        };
        app.insert_resource(request);
        PkvStore::new("test", "test3").clear().unwrap();
        let mut pkv = PkvStore::new("test", "test3");
        let mut tags = HashMap::new();
        tags.insert(doc_id, vec!["test_tag_2".to_string()]);
        pkv.set("tags", &tags).unwrap();
        app.add_event::<SaveStoreEvent>();
        app.insert_resource(pkv);
        app.insert_resource(app_state);

        // Run systems
        app.update();

        // Assertions
        // Check that the document was saved to the PKV store
        let pkv = app.world.resource::<PkvStore>();
        let saved_docs: HashMap<ReflectableUuid, Doc> = pkv.get("docs").unwrap();
        assert_eq!(saved_docs.get(&doc_id).unwrap().name, "test_doc");
        assert!(saved_docs.get(&doc_id).unwrap().tabs[0].is_active);
        // Check that the tags were saved to the PKV store
        let saved_tags: HashMap<ReflectableUuid, Vec<String>> = pkv.get("tags").unwrap();
        let expected_tags = existing_tags;
        assert_eq!(saved_tags.get(&doc_id).unwrap(), &expected_tags);
        // Check that the name was saved to the PKV store
        let saved_names: HashMap<ReflectableUuid, String> = pkv.get("names").unwrap();
        assert_eq!(saved_names.get(&doc_id).unwrap(), "test_doc");
        // Check that the last_saved field was updated in the PKV store
        assert_eq!(pkv.get::<ReflectableUuid>("last_saved").unwrap(), doc_id);
        // Check that the file was saved to the correct path
        let file_contents = std::fs::read_to_string(temp_file_path).unwrap();
        let saved_doc: Doc = serde_json::from_str(&file_contents).unwrap();
        assert_eq!(saved_doc.name, "test_doc");
        assert!(saved_doc.tabs[0].is_active);
    }
}

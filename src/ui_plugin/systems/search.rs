use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_cosmic_edit::get_cosmic_text;
use bevy_cosmic_edit::ActiveEditor;
use bevy_cosmic_edit::CosmicEdit;
use bevy_pkv::PkvStore;
use bevy_ui_borders::Outline;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use tantivy::collector::TopDocs;
use tantivy::query::BooleanQuery;
use tantivy::query::FuzzyTermQuery;
use tantivy::query::Occur;
use tantivy::ReloadPolicy;

use tantivy::schema::*;
use tantivy::Index;
use uuid::Uuid;

use crate::resources::AppState;
use crate::ui_plugin::NodeType;
use crate::utils::ReflectableUuid;
use crate::APP_NAME;
use crate::ORG_NAME;

use super::ui_helpers::SearchButton;
use super::ui_helpers::SearchText;
use super::ui_helpers::VeloNode;
use super::UiState;

pub struct SearchIndexState {
    pub index: Index,
    pub tabs_to_delete: HashSet<Uuid>,
    pub node_updates: HashMap<NodeSearchLocation, String>,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct NodeSearchLocation {
    pub doc_id: Uuid,
    pub tab_id: Uuid,
    pub node_id: Uuid,
}

pub fn search_box_click(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &SearchButton),
        (Changed<Interaction>, With<SearchButton>),
    >,
    mut search_query: Query<(&SearchText, Entity), With<SearchText>>,
    mut state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, node) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                primary_window.cursor.icon = CursorIcon::Text;
                *state = UiState::default();
                state.search_box_to_edit = Some(node.id);
                for (search_text, entity) in &mut search_query.iter_mut() {
                    if search_text.id == node.id {
                        commands.insert_resource(ActiveEditor {
                            entity: Some(entity),
                        });
                        break;
                    }
                }
            }
            Interaction::Hovered => {
                primary_window.cursor.icon = CursorIcon::Hand;
            }
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}

pub fn search_box_text_changed(
    text_query: Query<&CosmicEdit, With<SearchText>>,
    mut previous_search_text: Local<String>,
    mut app_state: ResMut<AppState>,
    pkv: Res<PkvStore>,
    mut velo_node_query: Query<(&mut Outline, &VeloNode, Entity), With<VeloNode>>,
) {
    let str = get_cosmic_text(&text_query.single().editor);
    if str != *previous_search_text {
        if !str.is_empty() {
            if let Some(index) = &app_state.search_index {
                let index = &index.index;
                let result = fuzzy_search(index, str.as_str());
                match result {
                    Ok(docs) => {
                        let node_ids: HashSet<ReflectableUuid> = docs
                            .clone()
                            .into_iter()
                            .map(|l| ReflectableUuid(l.node_id))
                            .collect();
                        highlight_search_match_nodes(&node_ids, &mut velo_node_query);
                        let doc_ids: HashSet<ReflectableUuid> = docs
                            .into_iter()
                            .map(|location| ReflectableUuid(location.doc_id))
                            .collect();
                        app_state.doc_list_ui = doc_ids;
                    }
                    Err(e) => info!("Error searching index {:?}", e),
                }
            }
        } else if let Ok(names) = pkv.get::<HashMap<ReflectableUuid, String>>("names") {
            highlight_search_match_nodes(&HashSet::new(), &mut velo_node_query);
            let keys_in_storage: Vec<_> = names.keys().collect();
            let keys_in_memory: Vec<_> = app_state.docs.keys().cloned().collect();
            let mut combined_keys = keys_in_memory;
            combined_keys.extend(keys_in_storage);
            app_state.doc_list_ui.extend(combined_keys);
        }
        *previous_search_text = str;
    }
}

pub fn init_search_index(mut app_state: ResMut<AppState>) {
    let dirs = directories::ProjectDirs::from("", ORG_NAME, APP_NAME);
    let path = match dirs.as_ref() {
        Some(dirs) => dirs.data_dir(),
        None => Path::new("."),
    }
    .to_path_buf();
    app_state.search_index = Some(SearchIndexState {
        index: initialize_search_index(path),
        node_updates: HashMap::new(),
        tabs_to_delete: HashSet::new(),
    });
}

pub fn initialize_search_index(dir: PathBuf) -> tantivy::Index {
    Index::open_in_dir(dir.clone()).unwrap_or_else(|_| {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("text", TEXT);
        schema_builder.add_text_field("full_text", STRING);
        schema_builder.add_text_field("doc_id", STRING | STORED);
        schema_builder.add_text_field("tab_id", STRING | STORED);
        schema_builder.add_text_field("node_id", STRING | STORED);
        let schema = schema_builder.build();
        Index::create_in_dir(dir, schema).unwrap()
    })
}

pub fn update_search_index(
    index: &Index,
    node_search_locations: &HashMap<NodeSearchLocation, String>,
) -> tantivy::Result<()> {
    let mut index_writer = index.writer(50_000_000)?;

    for (node_search_location, str) in node_search_locations.iter() {
        let term = tantivy::Term::from_field_text(
            index.schema().get_field("node_id").unwrap(),
            &node_search_location.node_id.to_string(),
        );
        index_writer.delete_term(term);

        let mut document = tantivy::Document::new();
        document.add_text(index.schema().get_field("text").unwrap(), str);
        document.add_text(index.schema().get_field("full_text").unwrap(), str);
        document.add_text(
            index.schema().get_field("doc_id").unwrap(),
            &node_search_location.doc_id.to_string(),
        );
        document.add_text(
            index.schema().get_field("tab_id").unwrap(),
            &node_search_location.tab_id.to_string(),
        );
        document.add_text(
            index.schema().get_field("node_id").unwrap(),
            &node_search_location.node_id.to_string(),
        );

        index_writer.add_document(document)?;
    }

    index_writer.commit()?;

    Ok(())
}

const MAX_SEARCH_RESULTS: usize = 1000;

pub fn clear_tabs_index(index: &Index, tab_ids: &HashSet<Uuid>) -> tantivy::Result<()> {
    let mut index_writer = index.writer(50_000_000)?;

    for tab_id in tab_ids {
        let term = tantivy::Term::from_field_text(
            index.schema().get_field("tab_id").unwrap(),
            &tab_id.to_string(),
        );
        index_writer.delete_term(term);
    }

    index_writer.commit()?;

    Ok(())
}

pub fn clear_doc_index(index: &Index, doc_id: &Uuid) -> tantivy::Result<()> {
    let mut index_writer = index.writer(50_000_000)?;

    let term = tantivy::Term::from_field_text(
        index.schema().get_field("doc_id").unwrap(),
        &doc_id.to_string(),
    );
    index_writer.delete_term(term);

    index_writer.commit()?;

    Ok(())
}

pub fn fuzzy_search(index: &Index, query: &str) -> tantivy::Result<Vec<NodeSearchLocation>> {
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;
    let searcher = reader.searcher();
    let normalized_query = query.to_lowercase();

    let schema = index.schema();
    let text_field = schema.get_field("text").unwrap();
    let full_text_field = schema.get_field("full_text").unwrap();
    let doc_id_field = schema.get_field("doc_id").unwrap();
    let tab_id_field = schema.get_field("tab_id").unwrap();
    let node_id_field = schema.get_field("node_id").unwrap();

    let text_term = Term::from_field_text(text_field, normalized_query.as_str());
    let query1 = FuzzyTermQuery::new(text_term, 2, true);

    let full_text_term = Term::from_field_text(full_text_field, normalized_query.as_str());
    let query2 = FuzzyTermQuery::new(full_text_term, 2, true);
    let query = BooleanQuery::new(vec![
        (Occur::Should, Box::new(query1)),
        (Occur::Should, Box::new(query2)),
    ]);

    let top_docs = searcher
        .search(&query, &(TopDocs::with_limit(MAX_SEARCH_RESULTS)))
        .unwrap();
    let ids: Vec<NodeSearchLocation> = top_docs
        .iter()
        .map(|(_, doc_address)| {
            let doc = searcher.doc(*doc_address).unwrap();
            let doc_id_value = doc.get_first(doc_id_field).unwrap();
            let tab_id_value = doc.get_first(tab_id_field).unwrap();
            let node_id_value = doc.get_first(node_id_field).unwrap();
            NodeSearchLocation {
                doc_id: Uuid::parse_str(doc_id_value.as_text().unwrap()).unwrap(),
                tab_id: Uuid::parse_str(tab_id_value.as_text().unwrap()).unwrap(),
                node_id: Uuid::parse_str(node_id_value.as_text().unwrap()).unwrap(),
            }
        })
        .collect();
    Ok(ids)
}

pub fn highlight_search_match_nodes(
    node_ids: &HashSet<ReflectableUuid>,
    velo_node_query: &mut Query<(&mut Outline, &VeloNode, Entity), With<VeloNode>>,
) {
    let highlight_color = Color::TEAL;
    let highlight_thickness = UiRect::all(Val::Px(4.));
    for (mut outline, node, _) in velo_node_query.iter_mut() {
        if node_ids.contains(&node.id) {
            outline.color = highlight_color;
            outline.thickness = highlight_thickness;
        } else if outline.color == highlight_color && outline.thickness == highlight_thickness {
            // revert
            match node.node_type {
                NodeType::Rect => {
                    outline.color = Color::rgb(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0);
                }
                NodeType::Circle => {
                    outline.color = Color::rgba(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0, 0.);
                }
            }

            outline.thickness = UiRect::all(Val::Px(1.));
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_fuzzy_search() {
        // Create a temporary directory for the index
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");

        // Initialize the index using the temporary directory
        let index = initialize_search_index(temp_dir.path().to_path_buf());
        let id1 = Uuid::new_v4();
        let text1 = "apple".to_string();
        let id2 = Uuid::new_v4();
        let text2 = "banana".to_string();
        let mut node_search_locations = HashMap::new();
        node_search_locations.insert(
            NodeSearchLocation {
                doc_id: id1,
                tab_id: Uuid::new_v4(),
                node_id: Uuid::new_v4(),
            },
            text1,
        );
        node_search_locations.insert(
            NodeSearchLocation {
                doc_id: id2,
                tab_id: Uuid::new_v4(),
                node_id: Uuid::new_v4(),
            },
            text2,
        );
        update_search_index(&index, &node_search_locations).unwrap();
        // Perform fuzzy search and assert the results
        let query = "appla";
        let result = fuzzy_search(&index, query).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].doc_id, id1);
        // Clean up the temporary directory
        temp_dir
            .close()
            .expect("Failed to remove temporary directory");
    }

    #[test]
    fn test_clear_tab() {
        // Create a temporary directory for the index
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");

        // Initialize the index using the temporary directory
        let index = initialize_search_index(temp_dir.path().to_path_buf());
        let doc_id = Uuid::new_v4();
        let tab_id = Uuid::new_v4();
        let text_1 = "example text 1".to_string();
        let text_2 = "example text 2".to_string();
        let mut node_search_locations = HashMap::new();
        node_search_locations.insert(
            NodeSearchLocation {
                doc_id,
                tab_id,
                node_id: Uuid::new_v4(),
            },
            text_1,
        );
        node_search_locations.insert(
            NodeSearchLocation {
                doc_id,
                tab_id,
                node_id: Uuid::new_v4(),
            },
            text_2,
        );
        update_search_index(&index, &node_search_locations).unwrap();

        let mut tab_ids = HashSet::new();
        tab_ids.insert(tab_id);
        // Clear the tab from the index
        clear_tabs_index(&index, &tab_ids).unwrap();

        // Perform a search and assert that the tab is not found
        let query = "example";
        let result = fuzzy_search(&index, query).unwrap();

        assert_eq!(result.len(), 0);

        // Clean up the temporary directory
        temp_dir
            .close()
            .expect("Failed to remove temporary directory");
    }

    #[test]
    fn test_clear_doc() {
        // Create a temporary directory for the index
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");

        // Initialize the index using the temporary directory
        let index = initialize_search_index(temp_dir.path().to_path_buf());
        let doc_id = Uuid::new_v4();
        let text_1 = "example text 1".to_string();
        let text_2 = "example text 2".to_string();
        let mut node_search_locations = HashMap::new();
        node_search_locations.insert(
            NodeSearchLocation {
                doc_id,
                tab_id: Uuid::new_v4(),
                node_id: Uuid::new_v4(),
            },
            text_1,
        );
        node_search_locations.insert(
            NodeSearchLocation {
                doc_id,
                tab_id: Uuid::new_v4(),
                node_id: Uuid::new_v4(),
            },
            text_2,
        );
        update_search_index(&index, &node_search_locations).unwrap();

        // Clear the document from the index
        clear_doc_index(&index, &doc_id).unwrap();

        // Perform a search and assert that the document is not found
        let query = "example";
        let result = fuzzy_search(&index, query).unwrap();

        assert_eq!(result.len(), 0);

        // Clean up the temporary directory
        temp_dir
            .close()
            .expect("Failed to remove temporary directory");
    }
}

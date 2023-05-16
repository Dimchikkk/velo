use bevy::prelude::ResMut;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use tantivy::collector::TopDocs;
use tantivy::query::FuzzyTermQuery;
use tantivy::ReloadPolicy;

use tantivy::schema::*;
use tantivy::Index;
use uuid::Uuid;

use crate::resources::AppState;
use crate::utils::ReflectableUuid;
pub struct SearchIndexState {
    pub index: Index,
    pub deleted_tabs: Vec<ReflectableUuid>,
    pub updated_nodes: HashMap<NodeSearchLocation, String>,
}

#[derive(Eq, PartialEq, Hash)]
pub struct NodeSearchLocation {
    pub doc_id: Uuid,
    pub tab_id: Uuid,
    pub node_id: Uuid,
}

pub fn init_search_index(mut app_state: ResMut<AppState>) {
    let dirs = directories::ProjectDirs::from("", "test", "velo");
    let path = match dirs.as_ref() {
        Some(dirs) => dirs.data_dir(),
        None => Path::new("."),
    }
    .to_path_buf();
    app_state.search_index = Some(SearchIndexState {
        index: initialize_search_index(path),
        deleted_tabs: vec![],
        updated_nodes: HashMap::new(),
    });
}

pub fn initialize_search_index(dir: PathBuf) -> tantivy::Index {
    Index::open_in_dir(dir.clone()).unwrap_or_else(|_| {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("text", TEXT);
        schema_builder.add_text_field("doc_id", STRING | STORED);
        schema_builder.add_text_field("tab_id", STRING | STORED);
        schema_builder.add_text_field("node_id", STRING | STORED);
        let schema = schema_builder.build();
        Index::create_in_dir(dir, schema).unwrap()
    })
}

pub fn update_search_index(
    index: &Index,
    meta: NodeSearchLocation,
    text: &str,
) -> tantivy::Result<()> {
    let mut index_writer = index.writer(50_000_000)?;

    let term = tantivy::Term::from_field_text(
        index.schema().get_field("node_id").unwrap(),
        &meta.node_id.to_string(),
    );
    index_writer.delete_term(term);

    let mut document = tantivy::Document::new();
    document.add_text(index.schema().get_field("text").unwrap(), text);
    document.add_text(
        index.schema().get_field("doc_id").unwrap(),
        &meta.doc_id.to_string(),
    );
    document.add_text(
        index.schema().get_field("tab_id").unwrap(),
        &meta.tab_id.to_string(),
    );
    document.add_text(
        index.schema().get_field("node_id").unwrap(),
        &meta.node_id.to_string(),
    );

    index_writer.add_document(document)?;

    index_writer.commit()?;

    Ok(())
}

const MAX_SEARCH_RESULTS: usize = 1000;

pub fn clear_tab_index(index: &Index, tab_id: &Uuid) -> tantivy::Result<()> {
    let mut index_writer = index.writer(50_000_000)?;

    let term = tantivy::Term::from_field_text(
        index.schema().get_field("tab_id").unwrap(),
        &tab_id.to_string(),
    );
    index_writer.delete_term(term);

    index_writer.commit()?;

    Ok(())
}

pub fn clear_doc_index(index: &Index, tab_id: &Uuid) -> tantivy::Result<()> {
    let mut index_writer = index.writer(50_000_000)?;

    let term = tantivy::Term::from_field_text(
        index.schema().get_field("doc_id").unwrap(),
        &tab_id.to_string(),
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

    let schema = index.schema();
    let text_field = schema.get_field("text").unwrap();
    let doc_id_field = schema.get_field("doc_id").unwrap();
    let tab_id_field = schema.get_field("tab_id").unwrap();
    let node_id_field = schema.get_field("node_id").unwrap();

    let term = Term::from_field_text(text_field, query);
    let query = FuzzyTermQuery::new(term, 2, true);

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
        let text1 = "apple";
        let id2 = Uuid::new_v4();
        let text2 = "banana";
        update_search_index(
            &index,
            NodeSearchLocation {
                doc_id: id1,
                tab_id: Uuid::new_v4(),
                node_id: Uuid::new_v4(),
            },
            text1,
        )
        .unwrap();
        update_search_index(
            &index,
            NodeSearchLocation {
                doc_id: id2,
                tab_id: Uuid::new_v4(),
                node_id: Uuid::new_v4(),
            },
            text2,
        )
        .unwrap();

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
        let text_1 = "example text 1";
        let text_2 = "example text 2";
        update_search_index(
            &index,
            NodeSearchLocation {
                doc_id,
                tab_id,
                node_id: Uuid::new_v4(),
            },
            text_1,
        )
        .unwrap();
        update_search_index(
            &index,
            NodeSearchLocation {
                doc_id,
                tab_id,
                node_id: Uuid::new_v4(),
            },
            text_2,
        )
        .unwrap();

        // Clear the tab from the index
        clear_tab_index(&index, &tab_id).unwrap();

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
        let text_1 = "example text 1";
        let text_2 = "example text 2";
        update_search_index(
            &index,
            NodeSearchLocation {
                doc_id,
                tab_id: Uuid::new_v4(),
                node_id: Uuid::new_v4(),
            },
            text_1,
        )
        .unwrap();
        update_search_index(
            &index,
            NodeSearchLocation {
                doc_id,
                tab_id: Uuid::new_v4(),
                node_id: Uuid::new_v4(),
            },
            text_2,
        )
        .unwrap();

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

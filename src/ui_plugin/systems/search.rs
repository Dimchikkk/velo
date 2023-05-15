use std::path::PathBuf;
use tantivy::collector::TopDocs;
use tantivy::query::FuzzyTermQuery;
use tantivy::ReloadPolicy;

use tantivy::schema::*;
use tantivy::Index;

pub fn initialize_tantivy_index(dir: PathBuf) -> tantivy::Index {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("text", TEXT | STORED);
    schema_builder.add_text_field("id", STRING | STORED);
    let schema = schema_builder.build();
    Index::open_in_dir(dir.clone()).unwrap_or_else(|_| Index::create_in_dir(dir, schema).unwrap())
}

pub fn update_tantivy_index(index: &Index, id: String, text: &str) -> tantivy::Result<()> {
    let mut index_writer = index.writer(50_000_000)?;

    let term = tantivy::Term::from_field_text(index.schema().get_field("id").unwrap(), &id);
    index_writer.delete_term(term);

    let mut document = tantivy::Document::new();
    document.add_text(index.schema().get_field("id").unwrap(), id);
    document.add_text(index.schema().get_field("text").unwrap(), text);

    index_writer.add_document(document)?;

    index_writer.commit()?;

    Ok(())
}

const MAX_SEARCH_RESULTS: usize = 1000;

pub fn fuzzy_search(index: &Index, query: &str) -> tantivy::Result<Vec<String>> {
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;
    let searcher = reader.searcher();

    let schema = index.schema();
    let text_field = schema.get_field("text").unwrap();
    let id_field = schema.get_field("id").unwrap();

    let term = Term::from_field_text(text_field, query);
    let query = FuzzyTermQuery::new(term, 2, true);

    let top_docs = searcher
        .search(&query, &(TopDocs::with_limit(MAX_SEARCH_RESULTS)))
        .unwrap();
    let ids: Vec<String> = top_docs
        .iter()
        .map(|(_, doc_address)| {
            let doc = searcher.doc(*doc_address).unwrap();
            let id_value = doc.get_first(id_field).unwrap();
            id_value.as_text().unwrap().to_owned()
        })
        .collect();
    Ok(ids)
}

#[cfg(test)]
mod tests {

    use std::fs;

    use tempfile::TempDir;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_fuzzy_search() {
        // Create a temporary directory for the index
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");

        // Initialize the index using the temporary directory
        let index = initialize_tantivy_index(temp_dir.path().to_path_buf());
        let id1 = Uuid::new_v4().to_string();
        let text1 = "apple";
        let id2 = Uuid::new_v4().to_string();
        let text2 = "banana";
        update_tantivy_index(&index, id1.clone(), text1).unwrap();
        update_tantivy_index(&index, id2, text2).unwrap();

        // Perform fuzzy search and assert the results
        let query = "appla";
        let result = fuzzy_search(&index, query).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], id1);
        // Clean up the temporary directory
        fs::remove_dir_all(temp_dir).expect("Failed to remove temporary directory");
    }
}

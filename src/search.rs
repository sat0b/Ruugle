use crate::kvs::Kvs;
use crate::index::Index;
use crate::parse::tokenize;
use std::collections::HashSet;

pub struct Searcher {
    kvs: Kvs,
    index: Index,
}

#[derive(Debug)]
pub struct ResultField {
    id: u32,
    url: String,
}

#[derive(Debug)]
pub struct SearchResults {
    hits: u32,
    results: Vec<ResultField>,
}

impl Searcher {
    pub fn new(kvs: Kvs, index: Index) -> Searcher {
        Searcher { kvs, index }
    }
    pub fn search(&self, query: &str) -> SearchResults {
        let mut results = Vec::new();
        let queries = tokenize(query.to_string());
        let mut id_set = HashSet::new();
        for query in queries {
            let raw_results = self.index.search(&query).unwrap();
            for document_id in raw_results {
                let document = self.kvs.get(&document_id).expect(
                    &format!("Not found document_id {} in kvs", document_id));
                let cloned = document.clone();
                let result_field = ResultField { id: cloned.id, url: cloned.url };

                if id_set.contains(&cloned.id) {
                    continue
                } else {
                    results.push(result_field);
                    id_set.insert(cloned.id);
                }
            }
        }
        SearchResults { hits: results.len() as u32, results }
    }
}
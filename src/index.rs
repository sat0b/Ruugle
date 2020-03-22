use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write, Read};
use crate::parse::tokenize;

pub type Term = String;
pub type DocumentId = u32;
pub type DocumentCount = HashMap<DocumentId, i32>;
pub type Records = HashMap<Term, DocumentCount>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub id: DocumentId,
    pub url: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    size: u32,
    records: Records,
}

pub type Results = Vec<DocumentId>;

impl Index {
    pub fn new() -> Index {
        Index { size: 0, records: HashMap::new() }
    }

    pub fn insert(&mut self, document: Document) {
        let tokens = tokenize(document.content);
        for token in tokens {
            if self.records.contains_key(&token) {
                let document_count = self.records.get_mut(&token).unwrap();
                let count = document_count.get(&document.id).unwrap_or(&0).clone();
                document_count.insert(document.id, count + 1);
            } else {
                let mut document_count = DocumentCount::new();
                document_count.insert(document.id, 1);
                self.records.insert(token, document_count);
            }
        }
    }

    pub fn load(filename: &str) -> Index {
        let mut file = File::open(filename).unwrap();
        let mut records_str = String::new();
        file.read_to_string(&mut records_str).unwrap();
        let records: Records = serde_json::from_str(&records_str).unwrap();
        Index { size: records.len() as u32, records: records }
    }

    pub fn save(&self, filename: &str) {
        let mut f = BufWriter::new(File::create(filename).unwrap());
        let json_str = serde_json::to_string(&self.records).unwrap();
        f.write(json_str.as_bytes()).unwrap();
    }

    pub fn search(&self, term: &str) -> Option<Results> {
        let mut results = Results::new();
        match self.records.get(term) {
            Some(document_count) => {
                for document_id in document_count.keys() {
                    results.push(*document_id);
                }
                Some(results)
            }
            None => Some(results)
        }
    }
}

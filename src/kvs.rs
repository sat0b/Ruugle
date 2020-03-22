use std::fs::File;
use std::io::{BufWriter, Write, Read};
use std::collections::HashMap;

use crate::index::{DocumentId, Document};

pub struct Kvs {
    records: HashMap<DocumentId, Document>,
}

impl Kvs {
    pub fn new() -> Kvs {
        Kvs { records: HashMap::new() }
    }

    pub fn load(filename: &str) -> Kvs {
        let mut file = File::open(filename).unwrap();
        let mut records_str = String::new();
        file.read_to_string(&mut records_str).unwrap();
        let records = serde_json::from_str(&records_str).unwrap();
        Kvs { records }
    }

    pub fn set(&mut self, document: Document) {
        self.records.insert(document.id, document);
    }

    pub fn get(&self, document_id: &u32) -> Option<&Document> {
        self.records.get(document_id)
    }

    pub fn get_documents(&self) -> Vec<Document> {
        let mut documents = Vec::new();
        for (_document_id, document) in &self.records {
            documents.push(document.clone())
        }
        documents
    }

    pub fn save(&self, filename: &str) {
        let mut f = BufWriter::new(File::create(filename).unwrap());
        let json_str = serde_json::to_string(&self.records).unwrap();
        f.write(json_str.as_bytes()).unwrap();
    }
}



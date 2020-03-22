#[macro_use]
extern crate log;

use ruugle::index;
use ruugle::kvs::Kvs;
use clap::{App, Arg};


fn main() {
    env_logger::init();

    let matches = App::new("ruugle_indexer")
        .version("0.0.1")
        .author("Junichi Sato <sat0b.3ee@gmail.com>")
        .about("A indexer for ruugle engine")
        .arg(Arg::with_name("kvs_path")
            .short("k")
            .long("kvs_path")
            .default_value("kvs.json")
            .help("Kvs path"))
        .arg(Arg::with_name("index_path")
            .short("i")
            .long("index_path")
            .default_value("index.json")
            .help("Index path"))
        .get_matches();

    info!("Load data from kvs");
    let kvs_path = matches.value_of("kvs_path").unwrap();
    let kvs = Kvs::load(kvs_path);

    info!("Create Index");
    let index_path = matches.value_of("index_path").unwrap();
    let mut index = index::Index::new();

    for document in kvs.get_documents() {
        index.insert(document)
    }

    info!("Save index");
    index.save(index_path);
}

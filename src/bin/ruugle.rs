#[macro_use]
extern crate log;

use clap::{App, Arg};
use ruugle::index::Index;
use ruugle::search::Searcher;
use ruugle::kvs::Kvs;


fn main() {
    env_logger::init();

    let matches = App::new("ruugle")
        .version("0.0.1")
        .author("Junichi Sato <sat0b.3ee@gmail.com>")
        .about("A Search Engine in Rust")
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
        .arg(Arg::with_name("query")
            .index(1)
            .default_value("The")
            .help("query")
        )
        .get_matches();

    info!("Load index");
    let index_path = matches.value_of("index_path").unwrap();
    let index = Index::load(index_path);

    info!("Load data from kvs");
    let kvs_path = matches.value_of("kvs_path").unwrap();
    let kvs = Kvs::load(kvs_path);

    info!("Search Index");
    let searcher = Searcher::new(kvs, index);

    let query = matches.value_of("query").unwrap();
    info!("query: {}", query);

    let the_result = searcher.search(query);
    println!("{:?}", the_result);
}

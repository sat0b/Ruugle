#[macro_use]
extern crate log;

use ruugle::crawl::{crawl_pages, WebPage};
use ruugle::kvs::Kvs;
use clap::{Arg, App};
use ruugle::index::Document;

fn get_documents(web_pages: Vec<WebPage>) -> Vec<Document> {
    let mut documents = Vec::new();
    let mut count = 0;
    for web_page in web_pages {
        let document = Document { id: count, url: web_page.url, content: web_page.content };
        documents.push(document);
        count += 1;
    }
    documents
}

fn main() {
    env_logger::init();

    let matches = App::new("ruugle_crawler")
        .version("0.0.1")
        .author("Junichi Sato <sat0b.3ee@gmail.com>")
        .about("A Crawler for ruugle engine")
        .arg(Arg::with_name("kvs_path")
            .short("k")
            .long("kvs_path")
            .default_value("kvs.json")
            .help("Kvs path"))
        .arg(Arg::with_name("num_crawls")
            .long("num_crawls")
            .default_value("10")
            .help("The number of crawls"))
        .get_matches();

    let kvs_path = matches.value_of("kvs_path").unwrap();
    let num_crawls: i32 = matches.value_of("num_crawls")
        .unwrap()
        .parse()
        .unwrap();

    info!("Start crawl");
    let web_pages = crawl_pages(num_crawls);

    info!("Save data to json database");
    let mut kvs = Kvs::new();
    for document in get_documents(web_pages) {
        kvs.set(document);
    }
    kvs.save(kvs_path);
}

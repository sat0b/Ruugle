# ruugle

## crawler
```
A Crawler for ruugle engine

USAGE:
    crawler [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --kvs_path <kvs_path>        Kvs path [default: kvs.json]
        --num_crawls <num_crawls>    The number of crawls [default: 10]

```

### examples
Crawl Web pages
```
$ cargo run --bin crawler -- --kvs_path kvs.json --num_crawls 10
```

## indexer
```
A indexer for ruugle engine

USAGE:
    indexer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --index_path <index_path>    Index path [default: index.json]
    -k, --kvs_path <kvs_path>        Kvs path [default: kvs.json]

```

### examples
Build index from the crawled documents.
```
$ cargo run --bin indexer -- --kvs_path kvs.json --index_path index.json
```

## ruugle
```
A Search Engine in Rust

USAGE:
    ruugle [OPTIONS] [query]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --index_path <index_path>    Index path [default: index.json]
    -k, --kvs_path <kvs_path>        Kvs path [default: kvs.json]

ARGS:
    <query>    query [default: The]
```

### examples
```
$ cargo run --bin ruugle -- news
SearchResults { hits: 1, results: [ResultField { id: 1, url: "https://en.wikipedia.org/wiki/WSCB" }] }
```
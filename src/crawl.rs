use libxml::parser::Parser;
use libxml::xpath::Context;


#[derive(Debug)]
pub struct WebPage {
    pub url: String,
    pub content: String,
}

pub fn crawl() -> reqwest::Result<WebPage> {
    let res = reqwest::blocking::get("https://en.wikipedia.org/wiki/Special:Random").unwrap();

    let url = res.url().as_str().to_string();
    let mut content = res.text()?;
    content = get_text(content);
    let web_page = WebPage { url: url, content: content };
    Ok(web_page)
}

pub fn crawl_pages(num_crawls: i32) -> Vec<WebPage> {
    let mut web_pages = Vec::new();
    for _i in 1..num_crawls {
        let web_page = crawl().unwrap();
        web_pages.push(web_page);
    }
    web_pages
}

fn get_text(content: String) -> String {
    let parser = Parser::default();
    let doc = parser.parse_string(content).unwrap();
    let context = Context::new(&doc).unwrap();
    let nodes = context
        .evaluate("//p|b|a|h1|h2|h3|h4|h5/text()")
        .unwrap()
        .get_nodes_as_vec();

    let mut text = String::new();
    for node in nodes {
        text = text + " " + &node.get_content();
    }
    text = text.trim().to_string();
    text
}

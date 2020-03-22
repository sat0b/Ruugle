use regex::Regex;

pub fn tokenize(text: String) -> Vec<String> {
    let filtered = filter_stop_word(text);
    filtered.split_whitespace().map(String::from).collect()
}

fn filter_stop_word(content: String) -> String {
    let re = Regex::new("[\t|\n|\\|\"]").unwrap();
    re.replace_all(content.as_str(), " ").to_string()
}

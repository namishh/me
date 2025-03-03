use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

pub struct MarkdownCache {
    entries: HashMap<String, (String, Vec<(u8, String, String)>)>,
}

impl MarkdownCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, path: &str) -> Option<(String, Vec<(u8, String, String)>)> {
        self.entries.get(path).map(|(html, headings)| (html.clone(), headings.clone()))
    }

    pub fn set(&mut self, path: String, html: String, headings: Vec<(u8, String, String)>) {
        self.entries.insert(path, (html, headings));
    }
}

lazy_static! {
    pub static ref MARKDOWN_CACHE: Mutex<MarkdownCache> = Mutex::new(MarkdownCache::new());
}
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;
use lazy_static::lazy_static;

pub struct MarkdownCache {
    entries: HashMap<String, (SystemTime, String, Vec<(u8, String, String)>)>,
}

impl MarkdownCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get_if_fresh(&self, path: &str, current_modified: SystemTime) -> Option<(String, Vec<(u8, String, String)>)> {
        self.entries.get(path).and_then(|(cached_modified, html, headings)| {
            if *cached_modified == current_modified {
                Some((html.clone(), headings.clone()))
            } else {
                None
            }
        })
    }

    pub fn set(&mut self, path: String, modified: SystemTime, html: String, headings: Vec<(u8, String, String)>) {
        self.entries.insert(path, (modified, html, headings));
    }
}

lazy_static! {
    pub static ref MARKDOWN_CACHE: Mutex<MarkdownCache> = Mutex::new(MarkdownCache::new());
}
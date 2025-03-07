use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::sync::RwLock;
use rayon::prelude::*;
use crate::markdown::extract_frontmatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexedDocument {
    title: String,
    url: String,
    content: String,
    lowercase_content: String,
}

static SEARCH_INDEX: Lazy<RwLock<Vec<IndexedDocument>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub fn initialize_search_index(base_path: &Path) -> std::io::Result<()> {
    let mut documents = Vec::new();
    index_directory(base_path, base_path, &mut documents)?;
    
    let mut index = SEARCH_INDEX.write().unwrap();
    *index = documents;
    
    Ok(())
}

fn index_directory(base_path: &Path, current_dir: &Path, documents: &mut Vec<IndexedDocument>) -> std::io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        
        if file_name.starts_with('.') {
            continue;
        }
        
        if path.is_dir() {
            index_directory(base_path, &path, documents)?;
        } else if path.extension().map_or(false, |ext| ext == "md") {
            index_file(base_path, &path, documents)?;
        }
    }
    
    Ok(())
}

fn index_file(base_path: &Path, file_path: &Path, documents: &mut Vec<IndexedDocument>) -> std::io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    
    let (frontmatter, body) = extract_frontmatter(&content);
    
    let title = if let serde_json::Value::Object(ref map) = frontmatter {
        map.get("title")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                file_path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            })
            .to_string()
    } else {
        file_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    };
    
    // Skip draft posts
    if let serde_json::Value::Object(ref map) = frontmatter {
        if let Some(serde_json::Value::Bool(is_draft)) = map.get("draft") {
            if *is_draft {
                return Ok(());
            }
        }
    }
    let rel_path = file_path
        .strip_prefix(base_path)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to get relative path"))?;
    
    let url = rel_path
        .with_extension("")
        .to_string_lossy()
        .replace('\\', "/");
    
    documents.push(IndexedDocument {
        title,
        url,
        content: body.to_string(),
        lowercase_content: body.to_lowercase(),
    });
    
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,  
    pub contexts: Vec<ContextMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMatch {
    pub context: String,
    pub url: String,
}

pub fn search_content(query: &str) -> Vec<SearchResult> {
    let query = query.trim();
    
    if query.is_empty() {
        return Vec::new();
    }
    
    let lowercase_query = query.to_lowercase();
    let index = SEARCH_INDEX.read().unwrap();
    
    index.par_iter()
        .filter_map(|doc| {
            if doc.lowercase_content.contains(&lowercase_query) {
                let contexts = extract_all_contexts(&doc.content, &doc.lowercase_content, &doc.url, &lowercase_query);
                
                if !contexts.is_empty() {
                    Some(SearchResult {
                        title: doc.title.clone(),
                        url: doc.url.clone(),
                        contexts,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn extract_all_contexts(content: &str, lowercase_content: &str, base_url: &str, query: &str) -> Vec<ContextMatch> {
    let mut contexts = Vec::new();
    let mut last_pos = 0;
    let heading_regex = regex::Regex::new(r"^(#{1,4})\s+(.+)$").unwrap(); // Match #, ##, ###, or #### headings
    
    while let Some(pos) = lowercase_content[last_pos..].find(query) {
        let abs_pos = last_pos + pos;
        
        let mut last_heading = None;
        for line in content[..abs_pos].lines().rev() {
            if let Some(caps) = heading_regex.captures(line) {
                let heading_text = caps.get(2).unwrap().as_str().trim();
                last_heading = Some(
                    heading_text
                        .to_lowercase()
                        .replace(' ', "-")
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '-')
                        .collect::<String>()
                );
                break;
            }
        }
        
        let context_size = 40;
        
        let start_pos = lowercase_content[..abs_pos]
            .rfind(|c| c == '.' || c == '!' || c == '?')
            .map(|p| p + 1)
            .unwrap_or_else(|| abs_pos.saturating_sub(context_size));
        
        let end_pos = abs_pos + query.len() + 
            lowercase_content[abs_pos + query.len()..]
                .find(|c| c == '.' || c == '!' || c == '?')
                .unwrap_or_else(|| context_size.min(lowercase_content.len() - abs_pos - query.len()));
        
        let result = content[start_pos..end_pos].trim().to_string();
        
        let context_url = match last_heading {
            Some(heading) => format!("{}#{}", base_url, heading),
            None => base_url.to_string(),
        };
        
        contexts.push(ContextMatch {
            context: result,
            url: context_url,
        });
        
        last_pos = abs_pos + query.len() + 1;
        if last_pos >= lowercase_content.len() {
            break;
        }
    }
    
    contexts
}
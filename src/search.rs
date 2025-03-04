use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::sync::RwLock;
use rayon::prelude::*;
use crate::markdown::extract_frontmatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub context: String,
}

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
        
        // Skip hidden files and directories
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
                // Find context for the search term
                let context = extract_context(&doc.content, &doc.lowercase_content, &lowercase_query);
                
                Some(SearchResult {
                    title: doc.title.clone(),
                    url: doc.url.clone(),
                    context,
                })
            } else {
                None
            }
        })
        .collect()
}

fn extract_context(content: &str, lowercase_content: &str, query: &str) -> String {
    if let Some(pos) = lowercase_content.find(query) {
        let context_size = 40;
        
        let start_pos = lowercase_content[..pos]
            .rfind(|c| c == '.' || c == '!' || c == '?')
            .map(|p| p + 1)
            .unwrap_or_else(|| pos.saturating_sub(context_size));
        
        let end_pos = pos + query.len() + 
            lowercase_content[pos + query.len()..]
                .find(|c| c == '.' || c == '!' || c == '?')
                .unwrap_or_else(|| context_size.min(lowercase_content.len() - pos - query.len()));
        
        let mut result = content[start_pos..end_pos].trim().to_string();
        
        if start_pos > 0 {
            result = format!("...{}", result);
        }
        if end_pos < content.len() {
            result = format!("{}...", result);
        }
        
        result
    } else {
        let end = content.len().min(100);
        format!("{}...", &content[0..end].trim())
    }
}

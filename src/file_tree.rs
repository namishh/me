use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
}

pub fn build_file_tree(base: &Path, relative: &Path) -> Vec<FileNode> {
    let full_path = base.join(relative);
    let mut nodes = Vec::new();

    if let Ok(entries) = fs::read_dir(&full_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            if file_name.starts_with('.') {
                continue;
            }

            let is_dir = path.is_dir();
            let rel_path = relative.join(&file_name);
            let path_str = rel_path.to_string_lossy().replace('\\', "/");

            if is_dir {
                let children = build_file_tree(base, &rel_path);
                nodes.push(FileNode {
                    name: file_name,
                    path: path_str,
                    is_dir,
                    children,
                });
            } else if path.extension().map_or(false, |ext| ext == "md") {
                let mut name = String::new();
                let default_name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if let Ok(file) = File::open(&path) {
                    let reader = BufReader::new(file);
                    let mut in_frontmatter = false;
                    let mut found_title = false;

                    for line in reader.lines().filter_map(Result::ok) {
                        let trimmed_line = line.trim();
                        if trimmed_line == "---" {
                            if in_frontmatter {
                                break;
                            } else {
                                in_frontmatter = true;
                                continue;
                            }
                        }
                        if in_frontmatter {
                            if let Some((key, value)) = trimmed_line.split_once(':') {
                                let key = key.trim();
                                if key == "title" {
                                    name = value.trim().to_string();
                                    found_title = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !found_title || name.is_empty() {
                        name = default_name.clone();
                    }
                } else {
                    name = default_name.clone();
                }

                let trimmed_path = if path_str.ends_with(".md") {
                    path_str[..path_str.len() - 3].to_string()
                } else {
                    path_str.clone()
                };

                nodes.push(FileNode {
                    name,
                    path: trimmed_path,
                    is_dir,
                    children: Vec::new(),
                });
            }
        }
    }

    nodes.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    nodes
}

pub fn get_file_tree(file_tree: &Arc<Vec<FileNode>>) -> Vec<FileNode> {
    file_tree.as_ref().clone()
}
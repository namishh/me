use actix_web::{web, HttpResponse, Result};
use chrono::{DateTime, NaiveDate, Utc};
use regex::Regex;
use serde_json::Value as JsonValue;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use crate::state::AppState;
use crate::markdown::extract_frontmatter;
use rss::{ChannelBuilder, ItemBuilder};
use lazy_static::lazy_static;

lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"(\d{1,2})\s+(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)\s+(\d{4})").unwrap();
}

struct ContentItem {
    title: String,
    path: String,
    date: Option<DateTime<Utc>>,
    description: Option<String>,
}

pub async fn rss_feed(
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut items = Vec::new();
    collect_content_items(Path::new("content"), &mut items)?;
    
    items.sort_by(|a, b| {
        match (&a.date, &b.date) {
            (Some(a_date), Some(b_date)) => b_date.cmp(a_date),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.title.cmp(&b.title),
        }
    });
    
    let base_url = "https://namishh.me";
    
    let mut channel = ChannelBuilder::default()
        .title("namishh")
        .link(base_url)
        .description("Personal website and digital garden of namishh")
        .language(Some("en-us".to_string()))
        .last_build_date(Some(chrono::Utc::now().to_rfc2822()))
        .build();
        
    for item in items {
        let link = format!("{}/{}", base_url, item.path);
        let pub_date = item.date.map(|d| {
            d.format("%a, %d %b %Y").to_string()
        });
        
        let description = item.description.unwrap_or_else(|| "Read more about this content".to_string());
        
        let rss_item = ItemBuilder::default()
            .title(Some(item.title))
            .link(Some(link))
            .pub_date(pub_date)
            .description(description)
            .build();
            
        channel.items.push(rss_item);
    }

    let rss_content = channel.to_string();
    
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/xml; charset=utf-8"))
        .body(rss_content))
}

fn collect_content_items(dir: &Path, items: &mut Vec<ContentItem>) -> Result<(), actix_web::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read directory"))? {
            let entry = entry.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read directory entry"))?;
            let path = entry.path();
            
            // Skip hidden files and directories
            if path.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.starts_with('.'))
                .unwrap_or(false) {
                continue;
            }
            
            if path.is_dir() {
                collect_content_items(&path, items)?;
            } else if path.extension().map_or(false, |ext| ext == "md") {
                if let Some(item) = process_markdown_file(&path)? {
                    items.push(item);
                }
            }
        }
    }
    
    Ok(())
}

fn process_markdown_file(file_path: &Path) -> Result<Option<ContentItem>, actix_web::Error> {
    let content = fs::read_to_string(file_path)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read file"))?;
    
    let (frontmatter, _) = extract_frontmatter(&content);
    
    if let JsonValue::Object(map) = frontmatter {
        let title = map.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled")
            .to_string();
            
        let description = map.get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        let date_str = map.get("date")
            .and_then(|v| v.as_str());
            
        let date = date_str
            .and_then(|ds| parse_date(ds))
            .map(|nd| DateTime::<Utc>::from_naive_utc_and_offset(nd.and_hms_opt(0, 0, 0).unwrap(), Utc));
            
        // Skip drafts if indicated in frontmatter
        if let Some(JsonValue::Bool(is_draft)) = map.get("draft") {
            if *is_draft {
                return Ok(None);
            }
        }
        
        // Calculate relative path for URL
        let rel_path = file_path
            .strip_prefix("content")
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get relative path"))?;
        
        let url_path = rel_path
            .with_extension("")
            .to_string_lossy()
            .replace('\\', "/");
            
        Ok(Some(ContentItem {
            title,
            path: url_path.to_string(),
            date,
            description,
        }))
    } else {
        // Skip files without proper frontmatter
        Ok(None)
    }
}

fn parse_date(date_str: &str) -> Option<NaiveDate> {
    if let Some(captures) = DATE_REGEX.captures(date_str) {
        let day: u32 = captures.get(1)?.as_str().parse().ok()?;
        let month_str = captures.get(2)?.as_str();
        let year: i32 = captures.get(3)?.as_str().parse().ok()?;
        
        let month = match &month_str.to_lowercase()[..3] {
            "jan" => 1,
            "feb" => 2,
            "mar" => 3,
            "apr" => 4,
            "may" => 5,
            "jun" => 6,
            "jul" => 7,
            "aug" => 8,
            "sep" => 9,
            "oct" => 10,
            "nov" => 11,
            "dec" => 12,
            _ => return None,
        };
        
        NaiveDate::from_ymd_opt(year, month, day)
    } else {
        None
    }
}
use actix_web::{web, Error, HttpResponse};
use chrono::{DateTime, Utc};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, ImageEncoder};
use imageproc::drawing;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use ab_glyph::{FontRef, PxScale};

// Structure to hold tweet data
struct TweetData {
    author_name: String,
    author_username: String,
    profile_image_url: String,
    tweet_text: String,
    created_at: DateTime<Utc>,
    favorite_count: i64,
    has_media: bool,
    media_url: Option<String>,
}

// Function to parse the tweet data from the API response
async fn parse_tweet_data(json: Value) -> Option<TweetData> {
    let data = json.get("data")?;
    let user = data.get("user")?;
    
    // Extract basic tweet info
    let tweet_text = data.get("text")?.as_str()?.to_string();
    let created_at_str = data.get("created_at")?.as_str()?;
    let created_at = DateTime::parse_from_rfc3339(created_at_str)
        .ok()?
        .with_timezone(&Utc);
    
    // Extract user info
    let author_name = user.get("name")?.as_str()?.to_string();
    let author_username = user.get("screen_name")?.as_str()?.to_string();
    let profile_image_url = user.get("profile_image_url_https")?.as_str()?.to_string()
        .replace("_normal", ""); // Get higher resolution profile pic
    
    // Extract favorite count
    let favorite_count = data.get("favorite_count")?.as_i64()?;
    
    // Check for media
    let has_media = data.get("photos").map_or(false, |p| !p.as_array().unwrap_or(&vec![]).is_empty());
    let media_url = if has_media {
        data.get("photos")?.get(0)?.get("url")?.as_str().map(|s| s.to_string())
    } else {
        None
    };
    
    Some(TweetData {
        author_name,
        author_username,
        profile_image_url,
        tweet_text,
        created_at,
        favorite_count,
        has_media,
        media_url,
    })
}

fn format_tweet_date(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 365 {
        let years = diff.num_days() / 365;
        format!("{}y", years)
    } else if diff.num_days() > 30 {
        let months = diff.num_days() / 30;
        format!("{}mo", months)
    } else if diff.num_days() > 0 {
        format!("{}d", diff.num_days())
    } else if diff.num_hours() > 0 {
        format!("{}h", diff.num_hours())
    } else if diff.num_minutes() > 0 {
        format!("{}m", diff.num_minutes())
    } else {
        "now".to_string()
    }
}

async fn load_profile_image(url: &str) -> Option<DynamicImage> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .ok()?;
    
    let response = client.get(url)
        .send()
        .await
        .ok()?;
    
    if !response.status().is_success() {
        return None;
    }
    
    let bytes = response.bytes().await.ok()?;
    image::load_from_memory(&bytes).ok()
}

// Load tweet media image
async fn load_media_image(url: &str) -> Option<DynamicImage> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .ok()?;
    
    let response = client.get(url)
        .send()
        .await
        .ok()?;
    
    if !response.status().is_success() {
        return None;
    }
    
    let bytes = response.bytes().await.ok()?;
    image::load_from_memory(&bytes).ok()
}

// Circular crop for profile images
fn circle_crop(img: &DynamicImage) -> DynamicImage {
    let width = img.width();
    let height = img.height();
    let size = width.min(height);
    
    let mut result = DynamicImage::new_rgba8(size, size);
    
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;
    let radius = size as f32 / 2.0;
    
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance <= radius {
                let source_x = (x as f32 * width as f32 / size as f32) as u32;
                let source_y = (y as f32 * height as f32 / size as f32) as u32;
                
                if source_x < width && source_y < height {
                    let pixel = img.get_pixel(source_x, source_y);
                    result.put_pixel(x, y, pixel);
                }
            }
        }
    }
    
    result
}

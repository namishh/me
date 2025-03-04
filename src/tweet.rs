use chrono::{DateTime, Utc};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, ImageEncoder};
use reqwest::Client;
use serde_json::Value;
use std::{f32::consts::PI, time::Duration};
use ab_glyph::{Font, FontRef, Glyph, Point, PxScale, PxScaleFont, ScaleFont};
use sha2::Digest;

// Structure to hold tweet data
struct TweetData {
    author_name: String,
    author_username: String,
    profile_image_url: String,
    tweet_text: String,
    created_at: DateTime<Utc>,
    favorite_count: i64,
    reply_count: i64,
    media_url: Option<String>,
}

async fn parse_tweet_data(json: Value) -> Option<TweetData> {
    let data = json.get("data")?;
    let user = data.get("user")?;
    
    let tweet_text = data.get("text")?.as_str()?.to_string();
    let created_at_str = data.get("created_at")?.as_str()?;
    let created_at = DateTime::parse_from_rfc3339(created_at_str)
        .ok()?
        .with_timezone(&Utc);
    
    let author_name = user.get("name")?.as_str()?.to_string();
    let author_username = user.get("screen_name")?.as_str()?.to_string();
    let profile_image_url = user.get("profile_image_url_https")?.as_str()?.to_string()
        .replace("_normal", ""); // Get higher resolution profile pic
    
    let favorite_count = data.get("favorite_count")?.as_i64()?;
    let reply_count = data.get("conversation_count").and_then(|r| r.as_i64()).unwrap_or(0);
    
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
        reply_count,
        media_url,
    })
}

fn format_tweet_date(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 365 {
        let years = diff.num_days() / 365;
        format!("{}y ago", years)
    } else if diff.num_days() > 30 {
        let months = diff.num_days() / 30;
        format!("{}mo ago", months)
    } else if diff.num_days() > 0 {
        format!("{}d ago", diff.num_days())
    } else if diff.num_hours() > 0 {
        format!("{}h ago", diff.num_hours())
    } else if diff.num_minutes() > 0 {
        format!("{}m ago", diff.num_minutes())
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

fn circle_crop(img: &DynamicImage) -> DynamicImage {
    let width = img.width();
    let height = img.height();
    let size = width.min(height);
    
    let mut result = DynamicImage::new_rgba8(size, size);
    
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;
    let radius = size as f32 / 2.0;
    let aa_band = 1.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            let source_x = (x as f32 * width as f32 / size as f32) as u32;
            let source_y = (y as f32 * height as f32 / size as f32) as u32;
            
            if source_x < width && source_y < height {
                let source_pixel = img.get_pixel(source_x, source_y);
                
                let alpha = if distance <= radius - aa_band {
                    1.0
                } else if distance <= radius + aa_band {
                    (radius + aa_band - distance) / (2.0 * aa_band)
                } else {
                    0.0 
                };
                
                if alpha > 0.0 {
                    let r = (source_pixel[0] as f32 * alpha) as u8;
                    let g = (source_pixel[1] as f32 * alpha) as u8;
                    let b = (source_pixel[2] as f32 * alpha) as u8;
                    let a = (source_pixel[3] as f32 * alpha) as u8;
                    result.put_pixel(x, y, Rgba([r, g, b, a]));
                }
            }
        }
    }
    
    result
}

fn draw_text<F: Font>(image: &mut DynamicImage, text: &str, x: i32, y: i32, font: &impl ScaleFont<F>, color: Rgba<u8>) {
    let mut pen_position = Point { x: x as f32, y: y as f32 + font.height() };
    
    for c in text.chars() {
        let glyph_id = font.glyph_id(c);
        
        let glyph = Glyph {
            id: glyph_id,
            scale: font.scale(),
            position: pen_position,
        };
        
        if let Some(outlined_glyph) = font.outline_glyph(glyph) {
            let bounds = outlined_glyph.px_bounds();
            
            outlined_glyph.draw(|x, y, v| {
                if v > 0.01 {
                    let px = bounds.min.x as i32 + x as i32;
                    let py = bounds.min.y as i32 + y as i32;
                    
                    if px >= 0 && px < image.width() as i32 && py >= 0 && py < image.height() as i32 {
                        let mut pixel = image.get_pixel(px as u32, py as u32);
                        pixel[0] = ((1.0 - v) * pixel[0] as f32 + v * color[0] as f32) as u8;
                        pixel[1] = ((1.0 - v) * pixel[1] as f32 + v * color[1] as f32) as u8;
                        pixel[2] = ((1.0 - v) * pixel[2] as f32 + v * color[2] as f32) as u8;
                        image.put_pixel(px as u32, py as u32, pixel);
                    }
                }
            });
        }
        
        pen_position.x += font.h_advance(glyph_id);
        
        if let Some(next_char) = text.chars().nth(text.chars().position(|ch| ch == c).unwrap() + 1) {
            let next_glyph_id = font.glyph_id(next_char);
            pen_position.x += font.kern(glyph_id, next_glyph_id);
        }
    }
}

fn draw_wrapped_text<F: Font>(
    image: &mut DynamicImage, 
    text: &str, 
    x: i32, 
    y: i32, 
    max_width: i32,
    font: &impl ScaleFont<F>, 
    color: Rgba<u8>
) -> i32 {
    let mut current_y = y;
    let space_width = font.h_advance(font.glyph_id(' '));
    
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();
    let mut current_width = 0.0;
    
    for word in words {
        let mut word_width = 0.0;
        let mut prev_glyph_id = None;
        
        for c in word.chars() {
            let glyph_id = font.glyph_id(c);
            
            if let Some(prev_id) = prev_glyph_id {
                word_width += font.kern(prev_id, glyph_id);
            }
            
            word_width += font.h_advance(glyph_id);
            prev_glyph_id = Some(glyph_id);
        }
        
        if current_width > 0.0 && current_width + space_width + word_width > max_width as f32 {
            draw_text(image, &current_line, x, current_y, font, color);
            current_y += (font.height() * 1.5) as i32; 
            current_line.clear();
            current_width = 0.0;
        }
        
        if !current_line.is_empty() {
            current_line.push(' ');
            current_width += space_width;
        }
        current_line.push_str(word);
        current_width += word_width;
    }
    
    if !current_line.is_empty() {
        draw_text(image, &current_line, x, current_y, font, color);
        current_y += (font.height() * 1.5) as i32;
    }
    
    current_y
}

fn draw_horizontal_line(image: &mut DynamicImage, x: i32, y: i32, width: i32, color: Rgba<u8>) {
    for i in 0..width {
        let px = x + i;
        if px >= 0 && px < image.width() as i32 && y >= 0 && y < image.height() as i32 {
            image.put_pixel(px as u32, y as u32, color);
        }
    }
}

fn interpolate(color1: (u8, u8, u8), color2: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let r = ((1.0 - t) * color1.0 as f32 + t * color2.0 as f32).round() as u8;
    let g = ((1.0 - t) * color1.1 as f32 + t * color2.1 as f32).round() as u8;
    let b = ((1.0 - t) * color1.2 as f32 + t * color2.2 as f32).round() as u8;
    (r, g, b)
}

fn load_icon(path: &str) -> Option<DynamicImage> {
    image::open(path).ok()
}

fn calculate_tweet_height<'a>(
    tweet_data: &TweetData,
    text_scaled_font: &PxScaleFont<&FontRef<'a>>,
    media_image: Option<&DynamicImage>,
    content_width: i32,
) -> i32 {
    let profile_size = 60;
    let padding = 10;
    let header_height = profile_size + padding * 2;
    
    let text_height = {
        let space_width = text_scaled_font.h_advance(text_scaled_font.glyph_id(' '));
        let words: Vec<&str> = tweet_data.tweet_text.split_whitespace().collect();
        let mut current_width = 0.0;
        let mut line_count = 1;
        
        for word in words {
            let mut word_width = 0.0;
            let mut prev_glyph_id = None;
            
            for c in word.chars() {
                let glyph_id = text_scaled_font.glyph_id(c);
                if let Some(prev_id) = prev_glyph_id {
                    word_width += text_scaled_font.kern(prev_id, glyph_id);
                }
                word_width += text_scaled_font.h_advance(glyph_id);
                prev_glyph_id = Some(glyph_id);
            }
            
            if current_width > 0.0 && current_width + space_width + word_width > content_width as f32 {
                line_count += 1;
                current_width = word_width;
            } else {
                if !current_width.eq(&0.0) {
                    current_width += space_width;
                }
                current_width += word_width;
            }
        }
        
        (line_count as f32 * text_scaled_font.height() * 1.5) as i32
    };
    
    let media_height = if let Some(media) = media_image {
        let aspect_ratio = media.width() as f32 / media.height() as f32;
        let scaled_height = (content_width as f32 / aspect_ratio).round() as i32;
        scaled_height.min(400) + 10
    } else {
        0
    };
    
    let footer_height = 85;
    
    header_height + text_height + media_height + footer_height + padding * 2
}

fn clean_text(name: &str) -> String {
    name.chars()
        .filter(|c| {
            c.is_ascii_alphanumeric() || 
            [' ', '@', '#', '_', '-', '.', '(', ')', '[', ']', '{', '}', ':', ';', '<', '>', ',', '/', '\\', '`', '~', '$', '@', '%', '\n'].contains(c) // Allowed symbols
        })
        .collect()
}

pub async fn generate_tweet(id: &str, title_font: &FontRef<'_>, path_font: &FontRef<'_>) -> Result<Vec<u8>, String> {
    let title_scaled_font = title_font.as_scaled(PxScale::from(16.0));
    let path_scaled_font = path_font.as_scaled(PxScale::from(14.0));
    let text_scaled_font = path_font.as_scaled(PxScale::from(18.0));
    
    let tweet_url = format!("https://react-tweet.vercel.app/api/tweet/{}", id);
    let client = Client::builder().timeout(Duration::from_secs(10)).build().unwrap();

    let response = client.get(&tweet_url).send().await.expect("Failed to fetch tweet data");
    let body = response.bytes().await.expect("Failed to read response body");
    let json: Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    let tweet_data = parse_tweet_data(json).await.ok_or("Failed to parse tweet data")?;

    let profile_image = match load_profile_image(&tweet_data.profile_image_url).await {
        Some(img) => circle_crop(&img),
        None => return Err("Failed to load profile image".to_string()),
    };
    
    let media_image = if let Some(url) = &tweet_data.media_url {
        load_media_image(url).await
    } else {
        None
    };
    
    let mut hasher = sha2::Sha256::new();
    hasher.update(id.as_bytes());
    let hash = hasher.finalize();

    let color1 = (hash[0], hash[1], hash[2]);      
    let color2 = (hash[3], hash[4], hash[5]); 
    let theta = (hash[6] as f32 / 255.0) * 2.0 * PI;
    let v = (theta.cos(), theta.sin());         

    let width = 500;
    let x0: i32 = 20; 
    let y0: i32 = 20; 
    let padding: i32 = 10;
    let w: i32 = 460; 
    let content_width = w - (padding * 2);
    
    let calculated_height = calculate_tweet_height(
        &tweet_data, 
        &text_scaled_font, 
        media_image.as_ref(), 
        content_width
    );
    let h: i32 = calculated_height;
    let total_height = h + y0 * 2; 

    let mut image = ImageBuffer::new(width, total_height as u32);

    let r: i32 = 20;

    let tl_center_x = x0 + r; 
    let tl_center_y = y0 + r;
    let tr_center_x = x0 + w - r - 1;
    let tr_center_y = y0 + r;
    let bl_center_x = x0 + r;
    let bl_center_y = y0 + h - r - 1;
    let br_center_x = x0 + w - r - 1;
    let br_center_y = y0 + h - r - 1;

    for y in 0..total_height as u32 {
        for x in 0..width {
            let x_i32 = x as i32;
            let y_i32 = y as i32;

            // Define regions
            let in_horizontal_strip = x0 + r <= x_i32 && x_i32 < x0 + w - r && y0 <= y_i32 && y_i32 < y0 + h;
            let in_vertical_strip = x0 <= x_i32 && x_i32 < x0 + w && y0 + r <= y_i32 && y_i32 < y0 + h - r;

            let mut alpha = 0.0;
            let mut in_rounded_area = false;

            let dx_tl = (x_i32 - tl_center_x) as f32;
            let dy_tl = (y_i32 - tl_center_y) as f32;
            let dist_tl = (dx_tl * dx_tl + dy_tl * dy_tl).sqrt();
            if x_i32 <= tl_center_x && y_i32 <= tl_center_y {
                in_rounded_area = true;
                alpha = if dist_tl <= r as f32 {
                    1.0 
                } else if dist_tl <= (r + 1) as f32 {
                    (r + 1) as f32 - dist_tl 
                } else {
                    0.0 
                };
            }

            let dx_tr = (x_i32 - tr_center_x) as f32;
            let dy_tr = (y_i32 - tr_center_y) as f32;
            let dist_tr = (dx_tr * dx_tr + dy_tr * dy_tr).sqrt();
            if x_i32 >= tr_center_x && y_i32 <= tr_center_y && alpha == 0.0 {
                in_rounded_area = true;
                alpha = if dist_tr <= r as f32 {
                    1.0
                } else if dist_tr <= (r + 1) as f32 {
                    (r + 1) as f32 - dist_tr
                } else {
                    0.0
                };
            }

            let dx_bl = (x_i32 - bl_center_x) as f32;
            let dy_bl = (y_i32 - bl_center_y) as f32;
            let dist_bl = (dx_bl * dx_bl + dy_bl * dy_bl).sqrt();
            if x_i32 <= bl_center_x && y_i32 >= bl_center_y && alpha == 0.0 {
                in_rounded_area = true;
                alpha = if dist_bl <= r as f32 {
                    1.0
                } else if dist_bl <= (r + 1) as f32 {
                    (r + 1) as f32 - dist_bl
                } else {
                    0.0
                };
            }

            let dx_br = (x_i32 - br_center_x) as f32;
            let dy_br = (y_i32 - br_center_y) as f32;
            let dist_br = (dx_br * dx_br + dy_br * dy_br).sqrt();
            if x_i32 >= br_center_x && y_i32 >= br_center_y && alpha == 0.0 {
                in_rounded_area = true;
                alpha = if dist_br <= r as f32 {
                    1.0
                } else if dist_br <= (r + 1) as f32 {
                    (r + 1) as f32 - dist_br
                } else {
                    0.0
                };
            }

            if in_horizontal_strip || in_vertical_strip || (in_rounded_area && alpha > 0.0) {
                let fg_color = Rgba([255, 255, 255, 255]);
                if alpha < 1.0 && alpha > 0.0 {
                    let p_x = x as f32;
                    let p_y = y as f32;
                    let proj = p_x * v.0 + p_y * v.1;
                    let t = ((proj - min_proj(width as f32, total_height as f32, v)) / 
                            (max_proj(width as f32, total_height as f32, v) - 
                            min_proj(width as f32, total_height as f32, v))).clamp(0.0, 1.0);
                    let (r_bg, g_bg, b_bg) = interpolate(color1, color2, t);
                    let bg_color = Rgba([r_bg, g_bg, b_bg, 255]);
                    
                    let r = (alpha * fg_color[0] as f32 + (1.0 - alpha) * bg_color[0] as f32) as u8;
                    let g = (alpha * fg_color[1] as f32 + (1.0 - alpha) * bg_color[1] as f32) as u8;
                    let b = (alpha * fg_color[2] as f32 + (1.0 - alpha) * bg_color[2] as f32) as u8;
                    image.put_pixel(x, y, Rgba([r, g, b, 255]));
                } else {
                    image.put_pixel(x, y, fg_color); // Fully inside
                }
            } else {
                let p_x = x as f32;
                let p_y = y as f32;
                let proj = p_x * v.0 + p_y * v.1;
                let t = ((proj - min_proj(width as f32, total_height as f32, v)) / 
                        (max_proj(width as f32, total_height as f32, v) - 
                        min_proj(width as f32, total_height as f32, v))).clamp(0.0, 1.0);
                let (r, g, b) = interpolate(color1, color2, t);
                image.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }

    let mut dynamic_image = DynamicImage::ImageRgba8(image);

    let profile_size = 60;
    let profile_x = x0 + padding;
    let profile_y = y0 + padding;
    
    let resized_profile = profile_image.resize_exact(
        profile_size as u32, 
        profile_size as u32, 
        image::imageops::FilterType::Triangle
    );
    
    for y in 0..profile_size {
        for x in 0..profile_size {
            if y < resized_profile.height() as i32 && x < resized_profile.width() as i32 {
                let pixel = resized_profile.get_pixel(x as u32, y as u32);
                if pixel[3] > 0 {
                    dynamic_image.put_pixel(
                        (profile_x + x) as u32, 
                        (profile_y + y) as u32, 
                        pixel
                    );
                }
            }
        }
    }
    
    let text_x = profile_x + profile_size + 10;
    let display_name_y = profile_y + 20;
    let username_y = display_name_y + 15;

    draw_text(
        &mut dynamic_image, 
        &clean_text(&tweet_data.author_name), 
        text_x, 
        display_name_y, 
        &title_scaled_font, 
        Rgba([0, 0, 0, 255])
    );
    
    draw_text(
        &mut dynamic_image, 
        &format!("@{}", tweet_data.author_username), 
        text_x, 
        username_y, 
        &path_scaled_font, 
        Rgba([100, 100, 100, 255])
    );
    
    let tweet_text_y = profile_y + profile_size + 20;
    let text_end_y = draw_wrapped_text(
        &mut dynamic_image,
        &tweet_data.tweet_text, 
        x0 + padding,
        tweet_text_y,
        content_width,
        &text_scaled_font,
        Rgba([0, 0, 0, 255])
    );
    
    let mut current_y = text_end_y + 15;
    
    if let Some(media) = media_image {
        let media_width = content_width;
        let aspect_ratio = media.width() as f32 / media.height() as f32;
        let media_height = (media_width as f32 / aspect_ratio).round() as u32;
        
        let max_height = 400;
        let (final_width, final_height) = if media_height > max_height {
            let new_width = (max_height as f32 * aspect_ratio) as u32;
            (new_width, max_height)
        } else {
            (media_width as u32, media_height)
        };
        
        let resized_media = media.resize_exact(
            final_width,
            final_height,
            image::imageops::FilterType::Triangle
        );
        
        let media_x = x0 + padding + (content_width - resized_media.width() as i32) / 2;
        
        if current_y + final_height as i32 <= total_height - y0 {
            for y in 0..resized_media.height() {
                for x in 0..resized_media.width() {
                    let pixel = resized_media.get_pixel(x, y);
                    if pixel[3] > 0 {
                        let dest_x = media_x + x as i32;
                        let dest_y = current_y + y as i32;
                        if dest_x >= 0 && dest_x < width.try_into().unwrap()&& dest_y >= 0 && dest_y < total_height {
                            dynamic_image.put_pixel(
                                dest_x as u32, 
                                dest_y as u32, 
                                pixel
                            );
                        }
                    }
                }
            }
            current_y += final_height as i32 + 15;
        }
    }
    
    let date_text = format_tweet_date(tweet_data.created_at);
    let date_y = current_y + 10;
    if date_y < total_height - y0 {
        draw_text(
            &mut dynamic_image, 
            &date_text, 
            x0 + padding, 
            date_y, 
            &path_scaled_font, 
            Rgba([100, 100, 100, 255])
        );
    }
    
    let separator_y = date_y + 25;
    if separator_y < total_height - y0 {
        draw_horizontal_line(
            &mut dynamic_image,
            x0 + padding,
            separator_y,
            content_width,
            Rgba([220, 220, 220, 255])
        );
    }
    
    let metrics_y = separator_y + 20;
    if metrics_y < total_height - y0 {
        let like_icon_path = "static/_priv/icons/like.jpg";
        let like_icon = load_icon(like_icon_path);
        let mut likes_x = x0 + padding;
        
        if let Some(icon) = like_icon {
            let icon_size = 16;
            let resized_icon = icon.resize_exact(icon_size, icon_size, image::imageops::FilterType::Triangle);
            for y in 0..icon_size {
                for x in 0..icon_size {
                    let pixel = resized_icon.get_pixel(x, y);
                    if pixel[3] > 0 {
                        let dest_x = likes_x + x as i32;
                        let dest_y = metrics_y + y as i32 - icon_size as i32 / 2;
                        if dest_x >= 0 && dest_x < width.try_into().unwrap() && dest_y >= 0 && dest_y < total_height {
                            dynamic_image.put_pixel(dest_x as u32, dest_y as u32, pixel);
                        }
                    }
                }
            }
            likes_x += icon_size as i32 + 5;
        }
        
        draw_text(
            &mut dynamic_image,
            &format!("{}", tweet_data.favorite_count),
            likes_x,
            metrics_y - 8,
            &path_scaled_font,
            Rgba([100, 100, 100, 255])
        );
        
        let replies_x = likes_x + 30;
        let reply_icon_path = "static/_priv/icons/reply.jpg";
        let reply_icon = load_icon(reply_icon_path);
        
        if let Some(icon) = reply_icon {
            let icon_size = 16;
            let resized_icon = icon.resize_exact(icon_size, icon_size, image::imageops::FilterType::Triangle);
            for y in 0..icon_size {
                for x in 0..icon_size {
                    let pixel = resized_icon.get_pixel(x, y);
                    if pixel[3] > 0 {
                        let dest_x = replies_x + x as i32;
                        let dest_y = metrics_y + y as i32 - icon_size as i32 / 2;
                        if dest_x >= 0 && dest_x < width.try_into().unwrap() && dest_y >= 0 && dest_y < total_height {
                            dynamic_image.put_pixel(dest_x as u32, dest_y as u32, pixel);
                        }
                    }
                }
            }
        }
        
        draw_text(
            &mut dynamic_image,
            &format!("{}", tweet_data.reply_count),
            replies_x + 25,
            metrics_y - 8,
            &path_scaled_font,
            Rgba([100, 100, 100, 255])
        );
    }

    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            dynamic_image.as_bytes(),
            dynamic_image.width(),
            dynamic_image.height(),
            image::ExtendedColorType::Rgba8,
        )
        .expect("Failed to encode image");

    Ok(bytes)
}

fn min_proj(width: f32, height: f32, v: (f32, f32)) -> f32 {
    let proj1 = 0.0 * v.0 + 0.0 * v.1;
    let proj2 = width * v.0 + 0.0 * v.1;
    let proj3 = 0.0 * v.0 + height * v.1;
    let proj4 = width * v.0 + height * v.1;
    proj1.min(proj2).min(proj3).min(proj4)
}

fn max_proj(width: f32, height: f32, v: (f32, f32)) -> f32 {
    let proj1 = 0.0 * v.0 + 0.0 * v.1;
    let proj2 = width * v.0 + 0.0 * v.1;
    let proj3 = 0.0 * v.0 + height * v.1;
    let proj4 = width * v.0 + height * v.1;
    proj1.max(proj2).max(proj3).max(proj4)
}
use image::{DynamicImage, ImageBuffer, Rgba, ImageEncoder};
use imageproc::drawing;
use ab_glyph::{FontRef, PxScale};
use std::env;
use std::sync::{Arc, Mutex};
use lru::LruCache;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// global cache for OG images and preloaded backgrounds
lazy_static::lazy_static! {
    static ref OG_CACHE: Mutex<LruCache<u64, Vec<u8>>> = Mutex::new(LruCache::new(std::num::NonZero::new(100).unwrap())); 
    static ref BACKGROUND_IMAGES: Arc<BackgroundImages> = Arc::new(load_backgrounds());
}

struct BackgroundImages {
    notes: ImageBuffer<Rgba<u8>, Vec<u8>>,
    blog: ImageBuffer<Rgba<u8>, Vec<u8>>,
    poems: ImageBuffer<Rgba<u8>, Vec<u8>>,
    journal: ImageBuffer<Rgba<u8>, Vec<u8>>,
    others: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

fn load_backgrounds() -> BackgroundImages {
    let current_dir = env::current_dir().expect("Could not get current directory");
    let load_image = |path: &str| {
        image::open(current_dir.join(path))
            .map(|img| img.to_rgba8())
            .unwrap_or_else(|_| {
                let mut fallback = ImageBuffer::new(1200, 630);
                for pixel in fallback.pixels_mut() {
                    *pixel = Rgba([40, 40, 40, 255]);
                }
                fallback
            })
    };

    BackgroundImages {
        notes: load_image("static/_priv/og/notes.png"),
        blog: load_image("static/_priv/og/blog.png"),
        poems: load_image("static/_priv/og/poems.png"),
        journal: load_image("static/_priv/og/journal.png"),
        others: load_image("static/_priv/og/others.png"),
    }
}

// Helper to generate a cache key
fn generate_cache_key(title: &str, dir_path: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    title.hash(&mut hasher);
    dir_path.hash(&mut hasher);
    hasher.finish()
}

pub fn generate_content_og_image(
    title: &str,
    dir_path: &str,
    title_font: &FontRef,
    path_font: &FontRef,
    avatar: &Option<DynamicImage>,
) -> Vec<u8> {
    let cache_key = generate_cache_key(title, dir_path);
    {
        let mut cache = OG_CACHE.lock().unwrap();
        if let Some(cached) = cache.get(&cache_key) {
            return cached.clone();
        }
    }

    let bg = match dir_path {
        path if path.starts_with("notes") => &BACKGROUND_IMAGES.notes,
        path if path.starts_with("blog") => &BACKGROUND_IMAGES.blog,
        path if path.starts_with("journal") => &BACKGROUND_IMAGES.journal,
        path if path.starts_with("poems") => &BACKGROUND_IMAGES.poems,
        _ => &BACKGROUND_IMAGES.notes,
    };
    let mut img = bg.clone(); // Clone the preloaded background

    let text_color = Rgba([255, 255, 255, 255]);
    let title_scale = match title.len() {
        0..=20 => PxScale { x: 96.0, y: 96.0 },
        21..=30 => PxScale { x: 86.0, y: 86.0 },
        _ => PxScale { x: 72.0, y: 72.0 },
    };
    drawing::draw_text_mut(&mut img, text_color, 100, 200, title_scale, title_font, title);

    let path_scale = PxScale { x: 36.0, y: 36.0 };
    let path_text = format!("/{}", dir_path);
    drawing::draw_text_mut(&mut img, Rgba([240, 240, 240, 255]), 100, 500, path_scale, path_font, &path_text);

    if let Some(avatar_img) = avatar {
        static AVATAR_SIZE: u32 = 50;
        static MASK: once_cell::sync::Lazy<Vec<bool>> = once_cell::sync::Lazy::new(|| {
            let mut mask = vec![false; (AVATAR_SIZE * AVATAR_SIZE) as usize];
            let center = AVATAR_SIZE as f32 / 2.0;
            for y in 0..AVATAR_SIZE {
                for x in 0..AVATAR_SIZE {
                    let distance = ((x as f32 - center).powi(2) + (y as f32 - center).powi(2)).sqrt();
                    if distance <= center {
                        mask[(y * AVATAR_SIZE + x) as usize] = true;
                    }
                }
            }
            mask
        });

        let resized_avatar = avatar_img.resize_exact(AVATAR_SIZE, AVATAR_SIZE, image::imageops::FilterType::Lanczos3).to_rgba8();
        let avatar_x = 1200 - AVATAR_SIZE - 30;
        let avatar_y = 630 - AVATAR_SIZE - 30;

        for (i, &in_mask) in MASK.iter().enumerate() {
            if in_mask {
                let x = (i as u32 % AVATAR_SIZE) + avatar_x;
                let y = (i as u32 / AVATAR_SIZE) + avatar_y;
                if x < 1200 && y < 630 {
                    img.put_pixel(x, y, *resized_avatar.get_pixel(i as u32 % AVATAR_SIZE, i as u32 / AVATAR_SIZE));
                }
            }
        }
    }

    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            &img.into_raw(),
            1200,
            630,
            image::ExtendedColorType::Rgba8,
        )
        .expect("Failed to encode image");

    let mut cache = OG_CACHE.lock().unwrap();
    cache.put(cache_key, bytes.clone());
    bytes
}

pub fn generate_web_og_image(
    title: &str,
    subtitle: &str,
    title_font: &FontRef,
    path_font: &FontRef,
    avatar: &Option<DynamicImage>,
) -> Vec<u8> {
    let cache_key = generate_cache_key(title, subtitle);
    {
        let mut cache = OG_CACHE.lock().unwrap();
        if let Some(cached) = cache.get(&cache_key) {
            return cached.clone();
        }
    }

    let mut img = BACKGROUND_IMAGES.others.clone();

    let title_scale = PxScale { x: 120.0, y: 120.0 };
    drawing::draw_text_mut(&mut img, Rgba([255, 255, 255, 255]), 100, 200, title_scale, title_font, title);

    let subtitle_scale = PxScale { x: 48.0, y: 48.0 };
    drawing::draw_text_mut(&mut img, Rgba([240, 240, 240, 255]), 100, 320, subtitle_scale, path_font, subtitle);

    if let Some(avatar_img) = avatar {
        static AVATAR_SIZE: u32 = 150;
        static MASK: once_cell::sync::Lazy<Vec<bool>> = once_cell::sync::Lazy::new(|| {
            let mut mask = vec![false; (AVATAR_SIZE * AVATAR_SIZE) as usize];
            let center = AVATAR_SIZE as f32 / 2.0;
            for y in 0..AVATAR_SIZE {
                for x in 0..AVATAR_SIZE {
                    let distance = ((x as f32 - center).powi(2) + (y as f32 - center).powi(2)).sqrt();
                    if distance <= center {
                        mask[(y * AVATAR_SIZE + x) as usize] = true;
                    }
                }
            }
            mask
        });

        let resized_avatar = avatar_img.resize_exact(AVATAR_SIZE, AVATAR_SIZE, image::imageops::FilterType::Lanczos3).to_rgba8();
        let avatar_x = 1200 - AVATAR_SIZE - 80;
        let avatar_y = 80;

        for (i, &in_mask) in MASK.iter().enumerate() {
            if in_mask {
                let x = (i as u32 % AVATAR_SIZE) + avatar_x;
                let y = (i as u32 / AVATAR_SIZE) + avatar_y;
                if x < 1200 && y < 630 {
                    img.put_pixel(x, y, *resized_avatar.get_pixel(i as u32 % AVATAR_SIZE, i as u32 / AVATAR_SIZE));
                }
            }
        }
    }

    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            &img.into_raw(),
            1200,
            630,
            image::ExtendedColorType::Rgba8,
        )
        .expect("Failed to encode image");

    let mut cache = OG_CACHE.lock().unwrap();
    cache.put(cache_key, bytes.clone());
    bytes
}
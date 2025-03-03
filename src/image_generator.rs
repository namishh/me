use image::{DynamicImage, ImageBuffer, Rgba, ImageEncoder};
use imageproc::drawing;
use ab_glyph::{FontRef, PxScale};
use std::env;

pub fn generate_content_og_image(
    title: &str,
    dir_path: &str,
    title_font: &FontRef,
    path_font: &FontRef,
    avatar: &Option<DynamicImage>,
) -> Vec<u8> {

    let current_dir = env::current_dir().expect("Could not get current directory");
    let bg_image_path = match dir_path {
        path if path.starts_with("notes") => current_dir.join("static/_priv/og/notes.png"),
        path if path.starts_with("blog") => current_dir.join("static/_priv/og/blog.png"),
        path if path.starts_with("poems") => current_dir.join("static/_priv/og/poems.png"),
        _ => current_dir.join("static/_priv/og/notes.png"),
    };

    let mut img = if let Ok(bg_img) = image::open(&bg_image_path) {
        let resized_bg = bg_img.resize_to_fill(1200, 630, image::imageops::FilterType::Lanczos3);
        resized_bg.to_rgba8()
    } else {
        let mut fallback = ImageBuffer::new(1200, 630);
        for pixel in fallback.pixels_mut() {
            *pixel = Rgba([40, 40, 40, 255]);
        }
        fallback
    };

    let text_color = Rgba([255, 255, 255, 255]);
    let title_scale = if title.len() > 30 {
        PxScale { x: 72.0, y: 72.0 }
    } else if title.len() > 20 {
        PxScale { x: 86.0, y: 86.0 }
    } else {
        PxScale { x: 96.0, y: 96.0 }
    };
    drawing::draw_text_mut(&mut img, text_color, 100, 200, title_scale, title_font, title);
    let path_scale = PxScale { x: 36.0, y: 36.0 };
    let path_text = format!("/{}", dir_path);
    drawing::draw_text_mut(&mut img, Rgba([240, 240, 240, 255]), 100, 500, path_scale, path_font, &path_text);

    if let Some(avatar_img) = avatar {
        let avatar_size = 50;
        let avatar_x = 1200 - avatar_size - 30;
        let avatar_y = 630 - avatar_size - 30;
        let resized_avatar = avatar_img.resize_exact(
            avatar_size as u32,
            avatar_size as u32,
            image::imageops::FilterType::Lanczos3,
        );
        let avatar_rgba = resized_avatar.to_rgba8();
        for y in 0..avatar_size {
            for x in 0..avatar_size {
                let center_x = avatar_size as f32 / 2.0;
                let center_y = avatar_size as f32 / 2.0;
                let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                if distance <= center_x && (avatar_x + x) < 1200 && (avatar_y + y) < 630 {
                    let avatar_pixel = avatar_rgba.get_pixel(x as u32, y as u32);
                    img.put_pixel((avatar_x + x) as u32, (avatar_y + y) as u32, *avatar_pixel);
                }
            }
        }
    }

    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            &dynamic_img.to_rgba8().into_raw(),
            dynamic_img.width(),
            dynamic_img.height(),
            image::ExtendedColorType::Rgba8,
        )
        .expect("Failed to encode image");
    bytes
}

pub fn generate_web_og_image(
    title: &str,
    subtitle: &str,
    title_font: &FontRef,
    path_font: &FontRef,
    avatar: &Option<DynamicImage>,
) -> Vec<u8> {
    let current_dir = env::current_dir().expect("Could not get current directory");
    let bg_image_path = current_dir.join("static/_priv/og/others.png");

    let mut img = if let Ok(bg_img) = image::open(&bg_image_path) {
        let resized_bg = bg_img.resize_to_fill(1200, 630, image::imageops::FilterType::Lanczos3);
        resized_bg.to_rgba8()
    } else {
        let mut fallback = ImageBuffer::new(1200, 630);
        for (x, _, pixel) in fallback.enumerate_pixels_mut() {
            let gradient = (x as f32 / 1200.0 * 30.0) as u8;
            *pixel = Rgba([30 + gradient, 30 + gradient, 50 + gradient, 255]);
        }
        fallback
    };

    let title_scale = PxScale { x: 120.0, y: 120.0 };
    drawing::draw_text_mut(&mut img, Rgba([255, 255, 255, 255]), 100, 200, title_scale, title_font, title);
    let subtitle_scale = PxScale { x: 48.0, y: 48.0 };
    drawing::draw_text_mut(&mut img, Rgba([240, 240, 240, 255]), 100, 320, subtitle_scale, path_font, subtitle);

    if let Some(avatar_img) = avatar {
        let avatar_size = 150;
        let avatar_x = 1200 - avatar_size - 80;
        let avatar_y = 80;
        let resized_avatar = avatar_img.resize_exact(
            avatar_size as u32,
            avatar_size as u32,
            image::imageops::FilterType::Lanczos3,
        );
        let avatar_rgba = resized_avatar.to_rgba8();
        for y in 0..avatar_size {
            for x in 0..avatar_size {
                let center_x = avatar_size as f32 / 2.0;
                let center_y = avatar_size as f32 / 2.0;
                let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                if distance <= center_x && (avatar_x + x) < 1200 && (avatar_y + y) < 630 {
                    let avatar_pixel = avatar_rgba.get_pixel(x as u32, y as u32);
                    img.put_pixel((avatar_x + x) as u32, (avatar_y + y) as u32, *avatar_pixel);
                }
            }
        }
    }

    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            &dynamic_img.to_rgba8().into_raw(),
            dynamic_img.width(),
            dynamic_img.height(),
            image::ExtendedColorType::Rgba8,
        )
        .expect("Failed to encode image");
    bytes
}
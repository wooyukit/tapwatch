use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use once_cell::sync::Lazy;
use std::io::Cursor;

// Sprite sheet configuration
const SPRITE_SHEET_PATH: &str = "src/assets/dog_sprites.png";

// Frame layout in the sprite sheet (1024x1024 pixels, 4x4 grid)
const FRAME_WIDTH: u32 = 256;  // 1024 / 4
const FRAME_HEIGHT: u32 = 256; // 1024 / 4

// Frame positions (x, y, width, height)
pub struct FrameRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

// Define frame positions for each animation state
// Idle animation: Rows 1-2 (first 8 frames)
const IDLE_FRAMES: &[FrameRect] = &[
    // Row 1: 4 idle frames
    FrameRect { x: 0, y: 0, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH, y: 0, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 2, y: 0, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 3, y: 0, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    // Row 2: 4 more idle frames
    FrameRect { x: 0, y: FRAME_HEIGHT, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH, y: FRAME_HEIGHT, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 2, y: FRAME_HEIGHT, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 3, y: FRAME_HEIGHT, width: FRAME_WIDTH, height: FRAME_HEIGHT },
];

// Typing animation: Rows 3-4 (last 8 frames)
const TYPING_FRAMES: &[FrameRect] = &[
    // Row 3: 4 typing frames
    FrameRect { x: 0, y: FRAME_HEIGHT * 2, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH, y: FRAME_HEIGHT * 2, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 2, y: FRAME_HEIGHT * 2, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 3, y: FRAME_HEIGHT * 2, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    // Row 4: 4 more typing frames
    FrameRect { x: 0, y: FRAME_HEIGHT * 3, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH, y: FRAME_HEIGHT * 3, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 2, y: FRAME_HEIGHT * 3, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 3, y: FRAME_HEIGHT * 3, width: FRAME_WIDTH, height: FRAME_HEIGHT },
];

// Background color to make transparent (white/light gray)
const BG_THRESHOLD: u8 = 240; // Pixels with R,G,B all above this become transparent

// Load the sprite sheet once
static SPRITE_SHEET: Lazy<Option<DynamicImage>> = Lazy::new(|| {
    std::fs::read(SPRITE_SHEET_PATH)
        .ok()
        .and_then(|bytes| image::load_from_memory(&bytes).ok())
});

// Pre-extract frames as PNG bytes for iTerm2 display
static TYPING_FRAME_BYTES: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
    extract_frames(TYPING_FRAMES)
});

static IDLE_FRAME_BYTES: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
    extract_frames(IDLE_FRAMES)
});

fn extract_frames(frames: &[FrameRect]) -> Vec<Vec<u8>> {
    let Some(sheet) = SPRITE_SHEET.as_ref() else {
        return vec![];
    };

    frames
        .iter()
        .filter_map(|rect| {
            // Crop the frame from the sprite sheet
            let cropped = sheet.crop_imm(rect.x, rect.y, rect.width, rect.height);

            // Convert to RGBA and remove background
            let rgba = remove_background(&cropped.to_rgba8());

            // Center the content to prevent shifting during animation
            let centered = center_content(&rgba, FRAME_WIDTH, FRAME_HEIGHT);

            // Convert to PNG bytes
            let mut bytes = Vec::new();
            let mut cursor = Cursor::new(&mut bytes);
            DynamicImage::ImageRgba8(centered)
                .write_to(&mut cursor, ImageFormat::Png)
                .ok()?;
            Some(bytes)
        })
        .collect()
}

/// Remove white/light background by making those pixels transparent
fn remove_background(img: &RgbaImage) -> RgbaImage {
    let mut result = img.clone();

    for pixel in result.pixels_mut() {
        let Rgba([r, g, b, _a]) = *pixel;

        // If pixel is very light (near white), make it transparent
        if r > BG_THRESHOLD && g > BG_THRESHOLD && b > BG_THRESHOLD {
            *pixel = Rgba([r, g, b, 0]); // Fully transparent
        }
    }

    result
}

/// Find the bounding box of non-transparent content in an image
fn find_content_bounds(img: &RgbaImage) -> Option<(u32, u32, u32, u32)> {
    let (width, height) = img.dimensions();
    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0u32;
    let mut max_y = 0u32;

    for (x, y, pixel) in img.enumerate_pixels() {
        let Rgba([_, _, _, a]) = *pixel;
        if a > 10 {  // Non-transparent pixel
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    if max_x >= min_x && max_y >= min_y {
        Some((min_x, min_y, max_x - min_x + 1, max_y - min_y + 1))
    } else {
        None
    }
}

/// Center the content within a frame of the specified size
fn center_content(img: &RgbaImage, target_width: u32, target_height: u32) -> RgbaImage {
    let Some((content_x, content_y, content_w, content_h)) = find_content_bounds(img) else {
        return img.clone();
    };

    // Create new image with transparent background
    let mut result = RgbaImage::from_pixel(target_width, target_height, Rgba([0, 0, 0, 0]));

    // Calculate offset to center the content
    let offset_x = (target_width.saturating_sub(content_w)) / 2;
    let offset_y = (target_height.saturating_sub(content_h)) / 2;

    // Copy content to centered position
    for y in 0..content_h {
        for x in 0..content_w {
            let src_x = content_x + x;
            let src_y = content_y + y;
            let dst_x = offset_x + x;
            let dst_y = offset_y + y;

            if src_x < img.width() && src_y < img.height()
                && dst_x < target_width && dst_y < target_height
            {
                result.put_pixel(dst_x, dst_y, *img.get_pixel(src_x, src_y));
            }
        }
    }

    result
}

/// Check if sprite sheet is loaded
pub fn is_loaded() -> bool {
    SPRITE_SHEET.is_some()
}

/// Get a typing animation frame as PNG bytes
pub fn get_typing_frame(frame_index: usize) -> Option<&'static [u8]> {
    let frames = &*TYPING_FRAME_BYTES;
    if frames.is_empty() {
        return None;
    }
    let index = frame_index % frames.len();
    Some(&frames[index])
}

/// Get an idle animation frame as PNG bytes
pub fn get_idle_frame(frame_index: usize) -> Option<&'static [u8]> {
    let frames = &*IDLE_FRAME_BYTES;
    if frames.is_empty() {
        return None;
    }
    let index = frame_index % frames.len();
    Some(&frames[index])
}

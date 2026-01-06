use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use once_cell::sync::Lazy;
use std::io::Cursor;

// Sprite sheet configuration
const SPRITE_SHEET_PATH: &str = "src/assets/dog_sprites.png";

// Frame layout in the sprite sheet (1024x1040 pixels)
// Assuming 5 frames per row for typing, 3 frames for idle
const FRAME_WIDTH: u32 = 204;  // 1024 / 5
const FRAME_HEIGHT: u32 = 346; // 1040 / 3

// Frame positions (x, y, width, height)
pub struct FrameRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

// Define frame positions for each animation state
const TYPING_FRAMES: &[FrameRect] = &[
    // Row 1: 5 typing frames
    FrameRect { x: 0, y: 0, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH, y: 0, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 2, y: 0, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 3, y: 0, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 4, y: 0, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    // Row 2: 5 more typing frames
    FrameRect { x: 0, y: FRAME_HEIGHT, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH, y: FRAME_HEIGHT, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 2, y: FRAME_HEIGHT, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 3, y: FRAME_HEIGHT, width: FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: FRAME_WIDTH * 4, y: FRAME_HEIGHT, width: FRAME_WIDTH, height: FRAME_HEIGHT },
];

// Idle frames (row 3, larger frames)
const IDLE_FRAME_WIDTH: u32 = 341; // 1024 / 3
const IDLE_FRAMES: &[FrameRect] = &[
    FrameRect { x: 0, y: FRAME_HEIGHT * 2, width: IDLE_FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: IDLE_FRAME_WIDTH, y: FRAME_HEIGHT * 2, width: IDLE_FRAME_WIDTH, height: FRAME_HEIGHT },
    FrameRect { x: IDLE_FRAME_WIDTH * 2, y: FRAME_HEIGHT * 2, width: IDLE_FRAME_WIDTH, height: FRAME_HEIGHT },
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

            // Convert to PNG bytes
            let mut bytes = Vec::new();
            let mut cursor = Cursor::new(&mut bytes);
            DynamicImage::ImageRgba8(rgba)
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

use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::Cursor;

// Sprite sheet configuration
const KEY_SPRITE_SHEET_PATH: &str = "src/assets/key_sprites.png";

// Frame dimensions - adjust these after seeing the actual sprite sheet
const KEY_FRAME_WIDTH: u32 = 64;
const KEY_FRAME_HEIGHT: u32 = 64;

// Number of columns in the sprite sheet (keys per row)
const COLUMNS: u32 = 10;

// Background color threshold for transparency
const BG_THRESHOLD: u8 = 240;

// Key layout in the sprite sheet (row, col) for each character
// This maps characters to their position in the grid
// Adjust this mapping based on your actual sprite sheet layout
fn get_key_position(key: &str) -> Option<(u32, u32)> {
    // Default layout assumes:
    // Row 0: 0-9
    // Row 1: A-J
    // Row 2: K-T
    // Row 3: U-Z, special keys
    let key_upper = key.to_uppercase();
    let first_char = key_upper.chars().next()?;

    match first_char {
        '0'..='9' => {
            let col = first_char as u32 - '0' as u32;
            Some((0, col))
        }
        'A'..='J' => {
            let col = first_char as u32 - 'A' as u32;
            Some((1, col))
        }
        'K'..='T' => {
            let col = first_char as u32 - 'K' as u32;
            Some((2, col))
        }
        'U'..='Z' => {
            let col = first_char as u32 - 'U' as u32;
            Some((3, col))
        }
        // Special keys - adjust positions as needed
        _ => None,
    }
}

// Load the sprite sheet once
static KEY_SPRITE_SHEET: Lazy<Option<DynamicImage>> = Lazy::new(|| {
    std::fs::read(KEY_SPRITE_SHEET_PATH)
        .ok()
        .and_then(|bytes| image::load_from_memory(&bytes).ok())
});

// Pre-extract all key frames
static KEY_FRAMES: Lazy<HashMap<String, Vec<u8>>> = Lazy::new(|| {
    extract_all_key_frames()
});

fn extract_all_key_frames() -> HashMap<String, Vec<u8>> {
    let mut frames = HashMap::new();

    let Some(sheet) = KEY_SPRITE_SHEET.as_ref() else {
        return frames;
    };

    // Extract digits 0-9
    for digit in '0'..='9' {
        if let Some((row, col)) = get_key_position(&digit.to_string()) {
            if let Some(data) = extract_frame(sheet, row, col) {
                frames.insert(digit.to_string(), data);
            }
        }
    }

    // Extract letters A-Z
    for letter in 'A'..='Z' {
        if let Some((row, col)) = get_key_position(&letter.to_string()) {
            if let Some(data) = extract_frame(sheet, row, col) {
                frames.insert(letter.to_string(), data);
            }
        }
    }

    frames
}

fn extract_frame(sheet: &DynamicImage, row: u32, col: u32) -> Option<Vec<u8>> {
    let x = col * KEY_FRAME_WIDTH;
    let y = row * KEY_FRAME_HEIGHT;

    // Check bounds
    if x + KEY_FRAME_WIDTH > sheet.width() || y + KEY_FRAME_HEIGHT > sheet.height() {
        return None;
    }

    let cropped = sheet.crop_imm(x, y, KEY_FRAME_WIDTH, KEY_FRAME_HEIGHT);
    let rgba = remove_background(&cropped.to_rgba8());

    let mut bytes = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);
    DynamicImage::ImageRgba8(rgba)
        .write_to(&mut cursor, ImageFormat::Png)
        .ok()?;

    Some(bytes)
}

/// Remove white/light background by making those pixels transparent
fn remove_background(img: &RgbaImage) -> RgbaImage {
    let mut result = img.clone();

    for pixel in result.pixels_mut() {
        let Rgba([r, g, b, _a]) = *pixel;

        if r > BG_THRESHOLD && g > BG_THRESHOLD && b > BG_THRESHOLD {
            *pixel = Rgba([r, g, b, 0]);
        }
    }

    result
}

/// Check if key sprite sheet is loaded
pub fn is_loaded() -> bool {
    KEY_SPRITE_SHEET.is_some()
}

/// Get a key sprite as PNG bytes
pub fn get_key_sprite(key: &str) -> Option<&'static [u8]> {
    let key_upper = key.to_uppercase();
    KEY_FRAMES.get(&key_upper).map(|v| v.as_slice())
}

/// Get frame dimensions for display sizing
pub fn frame_dimensions() -> (u32, u32) {
    (KEY_FRAME_WIDTH, KEY_FRAME_HEIGHT)
}

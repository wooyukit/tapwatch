use base64::{engine::general_purpose::STANDARD, Engine};
use std::io::{self, Write};

/// Check if running in iTerm2
pub fn is_iterm2() -> bool {
    std::env::var("TERM_PROGRAM")
        .map(|v| v == "iTerm.app")
        .unwrap_or(false)
}

/// Check if terminal supports inline images (iTerm2, WezTerm, Mintty)
pub fn supports_inline_images() -> bool {
    if let Ok(term) = std::env::var("TERM_PROGRAM") {
        matches!(term.as_str(), "iTerm.app" | "WezTerm" | "mintty")
    } else {
        false
    }
}

/// Display an inline image in iTerm2
/// Format: ESC ] 1337 ; File = [args] : base64_data BEL
///
/// # Arguments
/// * `data` - Raw image bytes (PNG, GIF, JPG, etc.)
/// * `width` - Width in character cells (None = auto)
/// * `height` - Height in character cells (None = auto)
/// * `preserve_aspect` - Whether to preserve aspect ratio
pub fn display_image_inline(
    data: &[u8],
    width: Option<u32>,
    height: Option<u32>,
    preserve_aspect: bool,
) -> io::Result<()> {
    let encoded = STANDARD.encode(data);

    let mut args = vec![
        format!("size={}", data.len()),
        "inline=1".to_string(),
    ];

    if let Some(w) = width {
        args.push(format!("width={}", w));
    }
    if let Some(h) = height {
        args.push(format!("height={}", h));
    }
    if preserve_aspect {
        args.push("preserveAspectRatio=1".to_string());
    }

    let args_str = args.join(";");

    // OSC 1337 ; File = args : base64 BEL
    print!("\x1b]1337;File={args_str}:{encoded}\x07");
    io::stdout().flush()
}

/// Display image at specific cursor position
/// Moves cursor to position, displays image, then restores cursor
pub fn display_image_at_position(
    data: &[u8],
    row: u16,
    col: u16,
    width: Option<u32>,
    height: Option<u32>,
) -> io::Result<()> {
    // Save cursor position
    print!("\x1b7");
    // Move to target position (1-indexed)
    print!("\x1b[{};{}H", row + 1, col + 1);
    // Display image
    display_image_inline(data, width, height, true)?;
    // Restore cursor position
    print!("\x1b8");
    io::stdout().flush()
}

/// Clear a region and display image there
/// Useful for animation frames
pub fn display_image_region(
    data: &[u8],
    row: u16,
    col: u16,
    clear_width: u16,
    clear_height: u16,
    img_width: Option<u32>,
    img_height: Option<u32>,
) -> io::Result<()> {
    // Save cursor
    print!("\x1b7");

    // Clear the region first (fill with spaces)
    for r in 0..clear_height {
        print!("\x1b[{};{}H", row + r + 1, col + 1);
        print!("{}", " ".repeat(clear_width as usize));
    }

    // Move to position and display
    print!("\x1b[{};{}H", row + 1, col + 1);
    display_image_inline(data, img_width, img_height, true)?;

    // Restore cursor
    print!("\x1b8");
    io::stdout().flush()
}

// Placeholder GIF data (1x1 transparent pixel)
// This will be replaced with actual dog GIF
pub const PLACEHOLDER_GIF: &[u8] = &[
    0x47, 0x49, 0x46, 0x38, 0x39, 0x61, // GIF89a
    0x01, 0x00, 0x01, 0x00, // 1x1
    0x00, 0x00, 0x00, // Global color table flag = 0
    0x21, 0xF9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, // Graphics extension
    0x2C, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, // Image descriptor
    0x02, 0x01, 0x44, 0x00, // Image data
    0x3B // Trailer
];

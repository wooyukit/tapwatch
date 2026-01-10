use base64::{engine::general_purpose::STANDARD, Engine};
use std::io::{self, Write};

/// Display an inline image using iTerm2's OSC 1337 protocol
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

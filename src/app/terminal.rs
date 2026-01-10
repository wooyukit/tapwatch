use base64::{engine::general_purpose::STANDARD, Engine};
use std::io::{self, Write};

// Compact window size to fit dog + text
const FIT_WIDTH: u32 = 400;   // pixels - width for text display
const FIT_HEIGHT: u32 = 380;  // pixels - height for text + dog
const MARGIN: u32 = 20;       // margin from screen edge

/// Direction for window movement
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

/// Fit window and move to edge of main screen (single direction)
pub fn fit_and_move(direction: Direction) -> io::Result<()> {
    let script = match direction {
        Direction::Top => format!(
            r#"tell application "iTerm2"
                tell current window
                    set {{x, y, x2, y2}} to bounds
                    set winWidth to x2 - x
                    set bounds to {{x, {m}, x + {w}, {m} + {h}}}
                end tell
            end tell"#,
            m = MARGIN, w = FIT_WIDTH, h = FIT_HEIGHT
        ),
        Direction::Bottom => format!(
            r#"use framework "AppKit"
            set mainScreen to current application's NSScreen's mainScreen()
            set visFrame to mainScreen's visibleFrame()
            set screenHeight to (item 2 of item 1 of visFrame) + (item 2 of item 2 of visFrame)
            tell application "iTerm2"
                tell current window
                    set {{x, y, x2, y2}} to bounds
                    set bounds to {{x, screenHeight - {h} - {m}, x + {w}, screenHeight - {m}}}
                end tell
            end tell"#,
            m = MARGIN, w = FIT_WIDTH, h = FIT_HEIGHT
        ),
        Direction::Left => format!(
            r#"tell application "iTerm2"
                tell current window
                    set {{x, y, x2, y2}} to bounds
                    set bounds to {{{m}, y, {m} + {w}, y + {h}}}
                end tell
            end tell"#,
            m = MARGIN, w = FIT_WIDTH, h = FIT_HEIGHT
        ),
        Direction::Right => format!(
            r#"use framework "AppKit"
            set mainScreen to current application's NSScreen's mainScreen()
            set frame to mainScreen's frame()
            set screenWidth to item 1 of item 2 of frame
            tell application "iTerm2"
                tell current window
                    set {{x, y, x2, y2}} to bounds
                    set bounds to {{screenWidth - {w} - {m}, y, screenWidth - {m}, y + {h}}}
                end tell
            end tell"#,
            m = MARGIN, w = FIT_WIDTH, h = FIT_HEIGHT
        ),
    };
    run_applescript(&script)
}

fn run_applescript(script: &str) -> io::Result<()> {
    use std::process::Command;
    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok();
    Ok(())
}

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
    // Use locked stdout to ensure atomic write of the entire escape sequence
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "\x1b]1337;File={args_str}:{encoded}\x07")?;
    handle.flush()
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
    // Use locked stdout to ensure atomic write of cursor operations + image
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Save cursor position
    write!(handle, "\x1b7")?;
    // Move to target position (1-indexed)
    write!(handle, "\x1b[{};{}H", row + 1, col + 1)?;
    handle.flush()?;

    // Display image (will acquire its own lock, so release first)
    drop(handle);
    display_image_inline(data, width, height, true)?;

    // Restore cursor position
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "\x1b8")?;
    handle.flush()
}

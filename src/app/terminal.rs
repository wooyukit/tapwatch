use base64::{engine::general_purpose::STANDARD, Engine};
use std::io::{self, Write};

// Compact window size to fit dog + text
const FIT_WIDTH: u32 = 400;   // pixels - width for text display
const FIT_HEIGHT: u32 = 340;  // pixels - height for text + dog (14 rows)
const MARGIN: u32 = 20;       // margin from screen edge

/// Direction for window movement
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

/// Fit window and move to edge of current screen (the screen where the window is located)
pub fn fit_and_move(direction: Direction) -> io::Result<()> {
    // Helper script to find the screen containing the window's center point
    // Note: NSScreen uses Cocoa coords (bottom-left origin, y up)
    //       Window bounds use screen coords (top-left origin, y down)
    // Conversion: window_y = mainScreenHeight - cocoa_y - height
    let find_screen_script = r#"
        use framework "AppKit"

        -- Get main screen height for coordinate conversion
        set mainScreen to current application's NSScreen's mainScreen()
        set mainFrame to mainScreen's frame()
        set mainScreenHeight to item 2 of item 2 of mainFrame

        -- Get current window bounds (in window/screen coordinates)
        tell application "iTerm2"
            tell current window
                set {wx, wy, wx2, wy2} to bounds
            end tell
        end tell

        -- Calculate window center in window coords
        set winCenterX to (wx + wx2) / 2
        set winCenterY to (wy + wy2) / 2

        -- Convert window center Y to Cocoa coords for screen lookup
        set winCenterYCocoa to mainScreenHeight - winCenterY

        -- Find the screen containing this point
        set allScreens to current application's NSScreen's screens()
        set screenCount to count of allScreens

        set targetFrame to missing value
        set targetVisFrame to missing value

        repeat with i from 1 to screenCount
            set scr to item i of allScreens
            set scrFrame to scr's frame()
            set scrX to item 1 of item 1 of scrFrame
            set scrY to item 2 of item 1 of scrFrame
            set scrW to item 1 of item 2 of scrFrame
            set scrH to item 2 of item 2 of scrFrame

            -- Check if window center is within this screen (using Cocoa coords)
            if winCenterX >= scrX and winCenterX < (scrX + scrW) and winCenterYCocoa >= scrY and winCenterYCocoa < (scrY + scrH) then
                set targetFrame to scrFrame
                set targetVisFrame to scr's visibleFrame()
                exit repeat
            end if
        end repeat

        -- Fallback to main screen if not found
        if targetFrame is missing value then
            set targetFrame to mainFrame
            set targetVisFrame to mainScreen's visibleFrame()
        end if
    "#;

    let script = match direction {
        Direction::Top => format!(
            r#"{find_screen}

            -- Get visible frame in Cocoa coords
            set visOriginY to item 2 of item 1 of targetVisFrame
            set visHeight to item 2 of item 2 of targetVisFrame

            -- Top of visible area in Cocoa coords = visOriginY + visHeight
            -- Convert to window coords: mainScreenHeight - (visOriginY + visHeight)
            set topInWindowCoords to mainScreenHeight - visOriginY - visHeight

            tell application "iTerm2"
                tell current window
                    set {{x, y, x2, y2}} to bounds
                    -- Keep x position, move to top of current screen
                    set bounds to {{x, topInWindowCoords + {m}, x + {w}, topInWindowCoords + {m} + {h}}}
                end tell
            end tell"#,
            find_screen = find_screen_script,
            m = MARGIN, w = FIT_WIDTH, h = FIT_HEIGHT
        ),
        Direction::Bottom => format!(
            r#"{find_screen}

            -- Get visible frame in Cocoa coords
            set visOriginY to item 2 of item 1 of targetVisFrame

            -- Bottom of visible area in Cocoa coords = visOriginY
            -- Convert to window coords: mainScreenHeight - visOriginY
            set bottomInWindowCoords to mainScreenHeight - visOriginY

            tell application "iTerm2"
                tell current window
                    set {{x, y, x2, y2}} to bounds
                    -- Keep x position, move to bottom of current screen
                    set bounds to {{x, bottomInWindowCoords - {h} - {m}, x + {w}, bottomInWindowCoords - {m}}}
                end tell
            end tell"#,
            find_screen = find_screen_script,
            m = MARGIN, w = FIT_WIDTH, h = FIT_HEIGHT
        ),
        Direction::Left => format!(
            r#"{find_screen}

            -- X coordinates are same in both systems
            set visOriginX to item 1 of item 1 of targetVisFrame

            tell application "iTerm2"
                tell current window
                    set {{x, y, x2, y2}} to bounds
                    -- Move to left edge, keep y position
                    set bounds to {{visOriginX + {m}, y, visOriginX + {m} + {w}, y + {h}}}
                end tell
            end tell"#,
            find_screen = find_screen_script,
            m = MARGIN, w = FIT_WIDTH, h = FIT_HEIGHT
        ),
        Direction::Right => format!(
            r#"{find_screen}

            -- X coordinates are same in both systems
            set visOriginX to item 1 of item 1 of targetVisFrame
            set visWidth to item 1 of item 2 of targetVisFrame
            set screenRight to visOriginX + visWidth

            tell application "iTerm2"
                tell current window
                    set {{x, y, x2, y2}} to bounds
                    -- Move to right edge, keep y position
                    set bounds to {{screenRight - {w} - {m}, y, screenRight - {m}, y + {h}}}
                end tell
            end tell"#,
            find_screen = find_screen_script,
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

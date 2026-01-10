use base64::{engine::general_purpose::STANDARD, Engine};
use once_cell::sync::Lazy;
use std::io::{self, Read, Write};
use std::time::Duration;

/// Cache the result of terminal capability detection
static SUPPORTS_IMAGES: Lazy<bool> = Lazy::new(detect_inline_image_support);

/// Check if terminal supports inline images
/// Uses cached result for performance
pub fn supports_inline_images() -> bool {
    *SUPPORTS_IMAGES
}

/// Detect if terminal supports inline images by querying terminal capabilities
///
/// Detection methods (in order):
/// 1. TAPWATCH_INLINE_IMAGES env var (force enable/disable)
/// 2. Query terminal for graphics support using DA1 (Primary Device Attributes)
/// 3. Check for Sixel support in terminal response
fn detect_inline_image_support() -> bool {
    // Allow user to force enable/disable via environment variable
    if let Ok(val) = std::env::var("TAPWATCH_INLINE_IMAGES") {
        return matches!(val.to_lowercase().as_str(), "1" | "true" | "yes" | "on");
    }

    // Try to query terminal capabilities
    if let Ok(supports) = query_graphics_support() {
        return supports;
    }

    // Fallback: check common environment variables as hint
    check_env_hints()
}

/// Query terminal for graphics protocol support
/// Sends DA1 (Primary Device Attributes) and checks for Sixel (4) or other graphics support
fn query_graphics_support() -> io::Result<bool> {
    use std::os::unix::io::AsRawFd;

    // We need raw terminal access to read the response
    let stdin = io::stdin();
    let stdin_fd = stdin.as_raw_fd();

    // Save current terminal settings
    let original_termios = match get_termios(stdin_fd) {
        Ok(t) => t,
        Err(_) => return Ok(false), // Can't query, assume no support
    };

    // Set terminal to raw mode for reading response
    let mut raw_termios = original_termios;
    make_raw(&mut raw_termios);
    if set_termios(stdin_fd, &raw_termios).is_err() {
        return Ok(false);
    }

    // Send DA1 query: ESC [ c
    print!("\x1b[c");
    io::stdout().flush()?;

    // Read response with timeout
    let response = read_with_timeout(Duration::from_millis(100));

    // Restore terminal settings
    let _ = set_termios(stdin_fd, &original_termios);

    // Parse response - look for graphics capability indicators
    // DA1 response format: ESC [ ? Ps ; Ps ; ... c
    // Sixel graphics is indicated by parameter 4
    if let Ok(resp) = response {
        // Check for Sixel support (;4; or ;4c in response)
        if resp.contains(";4;") || resp.contains(";4c") {
            return Ok(true);
        }
        // Check for any response (indicates a capable terminal)
        // Many modern terminals that respond to DA1 support some image protocol
        if resp.contains("\x1b[?") && resp.contains("c") {
            // Terminal responded - likely supports inline images
            // This is a heuristic, but works for most modern terminals
            return Ok(true);
        }
    }

    Ok(false)
}

/// Check environment variable hints for terminal capabilities
fn check_env_hints() -> bool {
    // Check for Kitty (has its own graphics protocol)
    if std::env::var("KITTY_WINDOW_ID").is_ok() {
        return true;
    }

    // Check for WezTerm
    if std::env::var("WEZTERM_PANE").is_ok() {
        return true;
    }

    // Check TERM for specific patterns
    if let Ok(term) = std::env::var("TERM") {
        let term_lower = term.to_lowercase();
        if term_lower.contains("kitty") || term_lower.contains("ghostty") {
            return true;
        }
    }

    false
}

// Low-level terminal handling for capability queries
#[cfg(unix)]
fn get_termios(fd: i32) -> io::Result<libc::termios> {
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        if libc::tcgetattr(fd, &mut termios) == 0 {
            Ok(termios)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[cfg(unix)]
fn set_termios(fd: i32, termios: &libc::termios) -> io::Result<()> {
    unsafe {
        if libc::tcsetattr(fd, libc::TCSANOW, termios) == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[cfg(unix)]
fn make_raw(termios: &mut libc::termios) {
    unsafe {
        libc::cfmakeraw(termios);
    }
    // Set read timeout
    termios.c_cc[libc::VMIN] = 0;
    termios.c_cc[libc::VTIME] = 1; // 0.1 second timeout
}

#[cfg(unix)]
fn read_with_timeout(timeout: Duration) -> io::Result<String> {
    let mut buffer = [0u8; 256];
    let mut result = String::new();
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        match handle.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                result.push_str(&String::from_utf8_lossy(&buffer[..n]));
                // Check if we got a complete response (ends with 'c')
                if result.ends_with('c') {
                    break;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(_) => break,
        }
    }

    Ok(result)
}

#[cfg(not(unix))]
fn get_termios(_fd: i32) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Unsupported, "Not supported on this platform"))
}

#[cfg(not(unix))]
fn set_termios(_fd: i32, _termios: &()) -> io::Result<()> {
    Ok(())
}

#[cfg(not(unix))]
fn make_raw(_termios: &mut ()) {}

#[cfg(not(unix))]
fn read_with_timeout(_timeout: Duration) -> io::Result<String> {
    Ok(String::new())
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

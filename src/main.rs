mod app;
mod font;
mod iterm2;
mod key_spritesheet;
mod sprite;
mod spritesheet;
mod ui;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use rdev::{listen, EventType, Key};
use std::{
    io::{self, stdout},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::Duration,
};

/// Target frame rate for smooth animations
const FRAME_RATE: u64 = 30; // Reduced for stability
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FRAME_RATE);

fn main() -> io::Result<()> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Set up bounded channel for global key events (prevents backpressure)
    let (tx, rx) = mpsc::sync_channel::<String>(32);

    // Spawn global key listener thread
    thread::spawn(move || {
        let callback = move |event: rdev::Event| {
            if let EventType::KeyPress(key) = event.event_type {
                let key_str = key_to_string(key);
                // Use try_send to avoid blocking if channel is full
                let _ = tx.try_send(key_str);
            }
        };

        // Note: On macOS, this requires Accessibility permissions
        if let Err(_e) = listen(callback) {
            // Silently fail - the app will still work with terminal-only input
        }
    });

    // Main event loop
    let result = run_app(&mut terminal, &mut app, rx);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    println!("Thanks for using TapWatch! (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧");
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    rx: Receiver<String>,
) -> io::Result<()> {
    loop {
        // Draw UI
        terminal.draw(|frame| ui::draw(frame, app))?;

        // Check for global key events (non-blocking, drain up to 10 at a time)
        for _ in 0..10 {
            match rx.try_recv() {
                Ok(key) => app.on_key(key),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        // Poll for terminal events (with timeout for animation)
        if event::poll(FRAME_DURATION)? {
            if let Event::Key(key_event) = event::read()? {
                // Only handle key press events (not release)
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            app.quit();
                        }
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            app.quit();
                        }
                        KeyCode::Esc => {
                            app.quit();
                        }
                        _ => {}
                    }
                }
            }
        }

        // Update animation state
        app.tick();

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

/// Convert rdev Key to a display string
fn key_to_string(key: Key) -> String {
    match key {
        // Letters
        Key::KeyA => "A".to_string(),
        Key::KeyB => "B".to_string(),
        Key::KeyC => "C".to_string(),
        Key::KeyD => "D".to_string(),
        Key::KeyE => "E".to_string(),
        Key::KeyF => "F".to_string(),
        Key::KeyG => "G".to_string(),
        Key::KeyH => "H".to_string(),
        Key::KeyI => "I".to_string(),
        Key::KeyJ => "J".to_string(),
        Key::KeyK => "K".to_string(),
        Key::KeyL => "L".to_string(),
        Key::KeyM => "M".to_string(),
        Key::KeyN => "N".to_string(),
        Key::KeyO => "O".to_string(),
        Key::KeyP => "P".to_string(),
        Key::KeyQ => "Q".to_string(),
        Key::KeyR => "R".to_string(),
        Key::KeyS => "S".to_string(),
        Key::KeyT => "T".to_string(),
        Key::KeyU => "U".to_string(),
        Key::KeyV => "V".to_string(),
        Key::KeyW => "W".to_string(),
        Key::KeyX => "X".to_string(),
        Key::KeyY => "Y".to_string(),
        Key::KeyZ => "Z".to_string(),

        // Numbers
        Key::Num0 => "0".to_string(),
        Key::Num1 => "1".to_string(),
        Key::Num2 => "2".to_string(),
        Key::Num3 => "3".to_string(),
        Key::Num4 => "4".to_string(),
        Key::Num5 => "5".to_string(),
        Key::Num6 => "6".to_string(),
        Key::Num7 => "7".to_string(),
        Key::Num8 => "8".to_string(),
        Key::Num9 => "9".to_string(),

        // Special keys
        Key::Space => "␣".to_string(),
        Key::Return => "⏎".to_string(),
        Key::Tab => "⇥".to_string(),
        Key::Backspace => "⌫".to_string(),
        Key::Escape => "⎋".to_string(),
        Key::Delete => "⌦".to_string(),

        // Arrow keys
        Key::UpArrow => "↑".to_string(),
        Key::DownArrow => "↓".to_string(),
        Key::LeftArrow => "←".to_string(),
        Key::RightArrow => "→".to_string(),

        // Modifiers
        Key::ShiftLeft | Key::ShiftRight => "⇧".to_string(),
        Key::ControlLeft | Key::ControlRight => "⌃".to_string(),
        Key::Alt | Key::AltGr => "⌥".to_string(),
        Key::MetaLeft | Key::MetaRight => "⌘".to_string(),

        // Function keys
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),

        // Punctuation
        Key::Comma => ",".to_string(),
        Key::Dot => ".".to_string(),
        Key::Slash => "/".to_string(),
        Key::SemiColon => ";".to_string(),
        Key::Quote => "'".to_string(),
        Key::LeftBracket => "[".to_string(),
        Key::RightBracket => "]".to_string(),
        Key::BackSlash => "\\".to_string(),
        Key::Minus => "-".to_string(),
        Key::Equal => "=".to_string(),
        Key::BackQuote => "`".to_string(),

        // Catch-all for unknown keys
        Key::Unknown(code) => format!("?{}", code),
        _ => "?".to_string(),
    }
}

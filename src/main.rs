mod app;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use rdev::{listen, EventType, Key}; // Key needed for shift detection
use std::{
    io::{self, stdout},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, TryRecvError},
        Arc,
    },
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

    // Track Shift state across events
    let shift_held = Arc::new(AtomicBool::new(false));

    // Spawn global key listener thread
    thread::spawn(move || {
        let shift_state = shift_held;
        let callback = move |event: rdev::Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    // Track Shift state
                    if matches!(key, Key::ShiftLeft | Key::ShiftRight) {
                        shift_state.store(true, Ordering::SeqCst);
                        return; // Don't send Shift as a key
                    }

                    let is_shifted = shift_state.load(Ordering::SeqCst);
                    let key_str = app::keys::key_to_string(key, is_shifted);
                    // Use try_send to avoid blocking if channel is full
                    let _ = tx.try_send(key_str.into_owned());
                }
                EventType::KeyRelease(key) => {
                    // Track Shift release
                    if matches!(key, Key::ShiftLeft | Key::ShiftRight) {
                        shift_state.store(false, Ordering::SeqCst);
                    }
                }
                _ => {}
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
        // Get terminal size for dirty-state tracking
        let term_size = terminal.size()?;
        let terminal_size = (term_size.width, term_size.height);

        // Check if images need redrawing (before draw closure borrows app)
        let needs_image_redraw = app.needs_image_redraw(terminal_size);

        // Get elapsed time for effect animations
        let elapsed = app.get_elapsed();

        // Draw UI
        terminal.draw(|frame| app::ui::draw(frame, app, needs_image_redraw, elapsed))?;

        // Mark as rendered if we did redraw images
        if needs_image_redraw {
            app.mark_rendered(terminal_size);
        }

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
                        KeyCode::Left => {
                            // Fit and move to left edge
                            let _ = app::terminal::fit_and_move(app::terminal::Direction::Left);
                        }
                        KeyCode::Right => {
                            // Fit and move to right edge
                            let _ = app::terminal::fit_and_move(app::terminal::Direction::Right);
                        }
                        KeyCode::Up => {
                            // Fit and move to top edge
                            let _ = app::terminal::fit_and_move(app::terminal::Direction::Top);
                        }
                        KeyCode::Down => {
                            // Fit and move to bottom edge
                            let _ = app::terminal::fit_and_move(app::terminal::Direction::Bottom);
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

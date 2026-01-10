# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TapWatch is a terminal-based typing companion app that displays an animated dog character reacting to keystrokes. It captures global keyboard input and shows the last key pressed alongside an animated sprite.

## Build & Run Commands

```bash
cargo build          # Build the project
cargo run            # Run in iTerm2/WezTerm for sprite sheet support
cargo build --release  # Release build
```

**Requirements:**
- On macOS, grant Accessibility permissions for global key capture (rdev)
- Best experience in iTerm2 or WezTerm (inline image support)

## Architecture

```
src/
├── main.rs           # Event loop, global key capture via rdev, terminal setup
├── app.rs            # Application state, animation state machine (Idle↔Typing)
├── ui.rs             # Ratatui UI rendering, sprite display logic
├── iterm2.rs         # iTerm2 inline image protocol (OSC 1337)
├── spritesheet.rs    # Dog sprite sheet extraction (1024x1024, 4x4 grid)
├── sprite.rs         # Text-based sprite fallback (Unicode block chars)
└── assets/
    └── dog_sprites.png  # Dog animation (4x4 grid, 256x256 per frame)
```

### Key Design Decisions

**Global Key Capture:** Uses `rdev::listen` in a separate thread with bounded `sync_channel(32)` and `try_send` to prevent UI freezing from event backpressure.

**Sprite Sheet Processing:** Frames are extracted at startup using the `image` crate. White/light backgrounds (RGB > 240) are made transparent via `remove_background()`.

**Dual Display Mode:** Checks `spritesheet::is_loaded() && iterm2::supports_inline_images()` to use PNG sprites, otherwise falls back to text-based Unicode sprites.

**Animation State Machine:**
- `Idle` state: Very slow animation (10s per frame, 8 frames from rows 1-2)
- `Typing` state: Moderate animation (250ms per frame, 8 frames from rows 3-4)
- Transitions to Typing on keypress, returns to Idle after completing the 8-frame typing cycle

**Sprite Sheet Layout:**
- Dog sprites: 1024x1024px, 4x4 grid (256x256 per frame). Rows 1-2 = idle, Rows 3-4 = typing

**Key Display:** Uses `tui-big-text` crate with `PixelSize::Quadrant` for large, stylish character display. Special keys (Space, Enter, etc.) are converted to ASCII representations.

**Dirty-State Rendering:** The app tracks `last_rendered_state`, `last_rendered_frame`, `last_rendered_key`, and `last_terminal_size` to only redraw iTerm2 images when visual state actually changes, preventing flicker.

**Exit Controls:** Press `q`, `Esc`, or `Ctrl+C` to quit the application.

## Key Dependencies

- `ratatui` - Terminal UI framework
- `crossterm` - Terminal manipulation
- `rdev` - Global keyboard/mouse capture
- `image` - PNG processing and sprite extraction
- `once_cell` - Lazy static initialization for sprite data
- `tui-big-text` - Large pixel text rendering for key display

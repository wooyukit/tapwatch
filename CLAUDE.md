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
├── main.rs        # Event loop, global key capture via rdev, terminal setup
├── app.rs         # Application state, animation state machine (Idle→Typing→Typed→Idle)
├── ui.rs          # Ratatui UI rendering, sprite display logic
├── iterm2.rs      # iTerm2 inline image protocol (OSC 1337)
├── spritesheet.rs # PNG sprite sheet extraction with background removal
├── sprite.rs      # Text-based sprite fallback (Unicode block chars)
├── font.rs        # 5x7 pixel font for big key display
└── assets/
    └── dog_sprites.png  # Sprite sheet (1024x1040, 5 frames/row typing, 3 idle)
```

### Key Design Decisions

**Global Key Capture:** Uses `rdev::listen` in a separate thread with bounded `sync_channel(32)` and `try_send` to prevent UI freezing from event backpressure.

**Sprite Sheet Processing:** Frames are extracted at startup using the `image` crate. White/light backgrounds (RGB > 240) are made transparent via `remove_background()`.

**Dual Display Mode:** Checks `spritesheet::is_loaded() && iterm2::supports_inline_images()` to use PNG sprites, otherwise falls back to text-based Unicode sprites.

**Frame Layout:** Sprite sheet assumes 5 columns for typing frames (204x346px each) and 3 columns for idle frames (341x346px each).

## Key Dependencies

- `ratatui` - Terminal UI framework
- `crossterm` - Terminal manipulation
- `rdev` - Global keyboard/mouse capture
- `image` - PNG processing and sprite extraction
- `once_cell` - Lazy static initialization for sprite data

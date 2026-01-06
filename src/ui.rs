use crate::app::{AnimationState, App};
use crate::font::get_char_sprite;
use crate::iterm2;
use crate::sprite;
use crate::spritesheet;
use chrono::Local;
use once_cell::sync::Lazy;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph},
    Frame,
};

// Color Palette
const BG_BLUE: Color = Color::Rgb(100, 175, 255);
const BG_LIGHT: Color = Color::Rgb(150, 200, 255);
const WHITE: Color = Color::Rgb(255, 255, 255);
const CREAM: Color = Color::Rgb(255, 252, 245);
const TEXT_BLUE: Color = Color::Rgb(60, 120, 200);
const BUBBLE_COLOR: Color = Color::Rgb(180, 220, 255);

// Bubble positions
const BUBBLES: [(u16, u16); 10] = [
    (5, 3), (8, 8), (3, 13), (10, 18),
    (30, 4), (35, 9), (28, 14), (33, 19),
    (18, 2), (22, 20),
];

// GIF paths (user can place custom GIFs here)
const GIF_IDLE_PATH: &str = "assets/dog_idle.gif";
const GIF_TYPING_PATH: &str = "assets/dog_typing.gif";

// Cached GIF data (loaded once at first access)
static GIF_IDLE: Lazy<Option<Vec<u8>>> = Lazy::new(|| std::fs::read(GIF_IDLE_PATH).ok());
static GIF_TYPING: Lazy<Option<Vec<u8>>> = Lazy::new(|| std::fs::read(GIF_TYPING_PATH).ok());

/// Check if GIF mode is available
pub fn gif_mode_available() -> bool {
    iterm2::supports_inline_images() && (GIF_IDLE.is_some() || GIF_TYPING.is_some())
}

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Blue background
    let bg = Block::default().style(Style::default().bg(BG_BLUE));
    frame.render_widget(bg, area);

    // Draw floating bubbles
    draw_bubbles(frame, area, app);

    // Calculate centered content
    let content_width = 40u16.min(area.width.saturating_sub(4));
    let content_height = 24u16.min(area.height.saturating_sub(2));

    let h_pad = (area.width.saturating_sub(content_width)) / 2;
    let v_pad = (area.height.saturating_sub(content_height)) / 2;

    let content = Rect {
        x: area.x + h_pad,
        y: area.y + v_pad,
        width: content_width,
        height: content_height,
    };

    // Layout sections
    let chunks = Layout::vertical([
        Constraint::Length(3),   // Time bubble
        Constraint::Length(1),   // Spacer
        Constraint::Length(11),  // Dog sprite/GIF area
        Constraint::Length(1),   // Spacer
        Constraint::Length(7),   // Big key display
        Constraint::Min(0),      // Remaining
    ])
    .split(content);

    // Draw components
    draw_time_bubble(frame, chunks[0]);
    draw_dog(frame, chunks[2], app);
    draw_key_display(frame, chunks[4], app);
}

fn draw_bubbles(frame: &mut Frame, area: Rect, app: &App) {
    let bubble_chars = ["○", "◯", "◌", "•", "∘"];

    for (i, (base_x, base_y)) in BUBBLES.iter().enumerate() {
        let y_offset = (app.frame_count / 30) as u16;
        let y = if *base_y > y_offset % 25 {
            *base_y - (y_offset % 25)
        } else {
            area.height.saturating_sub(y_offset % 25 - *base_y)
        };

        if y < area.height && *base_x < area.width {
            let bubble = bubble_chars[(i + (app.frame_count / 15) as usize) % bubble_chars.len()];
            let color = if i % 2 == 0 { BUBBLE_COLOR } else { WHITE };

            let para = Paragraph::new(bubble)
                .style(Style::default().fg(color).bg(BG_BLUE));

            if let Some(rect) = safe_rect(*base_x, y, 1, 1, area) {
                frame.render_widget(para, rect);
            }
        }
    }
}

fn draw_time_bubble(frame: &mut Frame, area: Rect) {
    let time = Local::now().format("%H:%M").to_string();

    let bubble_width = 14u16;
    let bubble_x = area.x + (area.width.saturating_sub(bubble_width)) / 2;

    let top = "╭────────────╮";
    let mid = format!("│    {}    │", time);
    let bot = "╰────────────╯";

    let top_para = Paragraph::new(top)
        .style(Style::default().fg(WHITE).bg(BG_BLUE));
    frame.render_widget(top_para, Rect {
        x: bubble_x,
        y: area.y,
        width: bubble_width,
        height: 1,
    });

    let mid_para = Paragraph::new(mid)
        .style(Style::default().fg(TEXT_BLUE).bg(CREAM).add_modifier(Modifier::BOLD));
    frame.render_widget(mid_para, Rect {
        x: bubble_x,
        y: area.y + 1,
        width: bubble_width,
        height: 1,
    });

    let bot_para = Paragraph::new(bot)
        .style(Style::default().fg(WHITE).bg(BG_BLUE));
    frame.render_widget(bot_para, Rect {
        x: bubble_x,
        y: area.y + 2,
        width: bubble_width,
        height: 1,
    });
}

fn draw_dog(frame: &mut Frame, area: Rect, app: &App) {
    // Use sprite sheet if available and iTerm2 supports it
    if spritesheet::is_loaded() && iterm2::supports_inline_images() {
        // Display sprite sheet frame via iTerm2
        display_spritesheet_frame(area, app);
    } else {
        // Fall back to text sprite
        draw_dog_sprite(frame, area, app);
    }
}

fn display_spritesheet_frame(area: Rect, app: &App) {
    let frame_data = match app.animation_state {
        AnimationState::Typing => spritesheet::get_typing_frame(app.animation_frame),
        AnimationState::Idle | AnimationState::Typed => spritesheet::get_idle_frame(app.animation_frame),
    };

    if let Some(data) = frame_data {
        // Calculate center position for the sprite (larger size)
        let sprite_width = 20u16; // Character cells for the sprite
        let sprite_height = 10u16;
        let col = area.x + (area.width.saturating_sub(sprite_width)) / 2;
        let row = area.y;

        let _ = iterm2::display_image_at_position(
            data,
            row,
            col,
            Some(sprite_width as u32),
            Some(sprite_height as u32),
        );
    }
}

fn draw_dog_sprite(frame: &mut Frame, area: Rect, app: &App) {
    let sprite = match app.animation_state {
        AnimationState::Idle => sprite::PIXEL_IDLE,
        AnimationState::Typing => sprite::get_frame(true, app.animation_frame),
        AnimationState::Typed => sprite::PIXEL_IDLE,
    };

    let color = match app.animation_state {
        AnimationState::Typing => WHITE,
        _ => CREAM,
    };

    let sprite_width = sprite.first().map(|s| s.len()).unwrap_or(0) as u16;
    let sprite_x = area.x + (area.width.saturating_sub(sprite_width)) / 2;

    for (i, row) in sprite.iter().enumerate() {
        let para = Paragraph::new(*row)
            .style(Style::default().fg(color).bg(BG_BLUE));

        if (i as u16) < area.height {
            frame.render_widget(para, Rect {
                x: sprite_x,
                y: area.y + i as u16,
                width: sprite_width.min(area.width),
                height: 1,
            });
        }
    }
}

fn draw_key_display(frame: &mut Frame, area: Rect, app: &App) {
    let key_char = match &app.last_key {
        Some(k) => k.chars().next().unwrap_or('?'),
        None => '?',
    };

    let char_sprite = get_char_sprite(key_char);

    let box_width = 11u16;
    let box_x = area.x + (area.width.saturating_sub(box_width)) / 2;

    let is_active = app.last_key.is_some();
    let border_color = if is_active { WHITE } else { BUBBLE_COLOR };
    let bg_color = CREAM;
    let text_color = if is_active { TEXT_BLUE } else { BG_LIGHT };

    // Top border
    let top = format!("╭{}╮", "─".repeat(box_width as usize - 2));
    let top_para = Paragraph::new(top)
        .style(Style::default().fg(border_color).bg(BG_BLUE));
    frame.render_widget(top_para, Rect {
        x: box_x,
        y: area.y,
        width: box_width,
        height: 1,
    });

    // Character rows
    for (i, row) in char_sprite.iter().enumerate() {
        let content = format!("│{}│", row);
        let para = Paragraph::new(content)
            .style(Style::default().fg(text_color).bg(bg_color).add_modifier(Modifier::BOLD));

        frame.render_widget(para, Rect {
            x: box_x,
            y: area.y + 1 + i as u16,
            width: box_width,
            height: 1,
        });
    }

    // Bottom border
    let bot = format!("╰{}╯", "─".repeat(box_width as usize - 2));
    let bot_para = Paragraph::new(bot)
        .style(Style::default().fg(border_color).bg(BG_BLUE));
    frame.render_widget(bot_para, Rect {
        x: box_x,
        y: area.y + 6,
        width: box_width,
        height: 1,
    });
}

fn safe_rect(x: u16, y: u16, width: u16, height: u16, parent: Rect) -> Option<Rect> {
    if x >= parent.width || y >= parent.height {
        return None;
    }

    Some(Rect {
        x: parent.x + x,
        y: parent.y + y,
        width: width.min(parent.width - x),
        height: height.min(parent.height - y),
    })
}

/// Display GIF at position (call from main loop, not during ratatui draw)
pub fn display_gif_if_available(app: &App, row: u16, col: u16) {
    if !iterm2::supports_inline_images() {
        return;
    }

    let gif_data = match app.animation_state {
        AnimationState::Typing => GIF_TYPING.as_ref().or(GIF_IDLE.as_ref()),
        _ => GIF_IDLE.as_ref(),
    };

    if let Some(data) = gif_data {
        let _ = iterm2::display_image_at_position(data, row, col, Some(15), Some(8));
    }
}

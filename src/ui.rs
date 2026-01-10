use crate::app::{AnimationState, App};
use crate::sprite;
use crate::spritesheet;
use crate::terminal;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use tui_big_text::{BigText, PixelSize};

// Color Palette
const WHITE: Color = Color::Rgb(255, 255, 255);
const CREAM: Color = Color::Rgb(255, 252, 245);
const KEY_TEXT: Color = Color::Rgb(200, 200, 220);  // Key letter color - soft light gray-blue

pub fn draw(frame: &mut Frame, app: &App, needs_image_redraw: bool) {
    let area = frame.area();

    // Calculate centered content area
    let content_width = 50u16.min(area.width.saturating_sub(4));
    let content_height = 16u16.min(area.height.saturating_sub(2));

    let h_pad = (area.width.saturating_sub(content_width)) / 2;
    let v_pad = (area.height.saturating_sub(content_height)) / 2;

    let content = Rect {
        x: area.x + h_pad,
        y: area.y + v_pad,
        width: content_width,
        height: content_height,
    };

    // Horizontal layout: key on left, dog on right (close together)
    let chunks = Layout::horizontal([
        Constraint::Length(8),  // Key display (left, compact)
        Constraint::Min(20),    // Dog sprite (right, flexible)
    ])
    .split(content);

    // Draw components (only redraw images when state changed)
    // Draw dog first, then key overlay
    draw_dog(frame, chunks[1], app, needs_image_redraw);

    // Only show key if one has been pressed
    if app.last_key.is_some() {
        draw_key_display(frame, chunks[0], app, needs_image_redraw);
    }
}

fn draw_dog(frame: &mut Frame, area: Rect, app: &App, needs_image_redraw: bool) {
    // Use sprite sheet if available and terminal supports inline images
    if spritesheet::is_loaded() && terminal::supports_inline_images() {
        // Only redraw image when state changed to prevent flashing
        if needs_image_redraw {
            display_spritesheet_frame(area, app);
        }
    } else {
        // Text sprites always render through ratatui (uses differential rendering)
        draw_dog_sprite(frame, area, app);
    }
}

fn display_spritesheet_frame(area: Rect, app: &App) {
    let frame_data = match app.animation_state {
        AnimationState::Typing => spritesheet::get_typing_frame(app.typing_frame),
        AnimationState::Idle => spritesheet::get_idle_frame(app.idle_frame),
    };

    if let Some(data) = frame_data {
        // Position sprite at left side of area (close to key)
        let sprite_width = 20u16;
        let sprite_height = 10u16;
        let col = area.x;  // Align to left
        let row = area.y + (area.height.saturating_sub(sprite_height)) / 2;

        let _ = terminal::display_image_at_position(
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
        AnimationState::Typing => sprite::get_frame(true, app.typing_frame),
    };

    let color = match app.animation_state {
        AnimationState::Typing => WHITE,
        _ => CREAM,
    };

    let sprite_width = sprite.first().map(|s| s.len()).unwrap_or(0) as u16;
    let sprite_height = sprite.len() as u16;
    let sprite_x = area.x;  // Align to left (close to key)
    let sprite_y = area.y + (area.height.saturating_sub(sprite_height)) / 2;

    for (i, row) in sprite.iter().enumerate() {
        let para = Paragraph::new(*row)
            .style(Style::default().fg(color));

        let y_pos = sprite_y + i as u16;
        if y_pos < area.y + area.height {
            frame.render_widget(para, Rect {
                x: sprite_x,
                y: y_pos,
                width: sprite_width.min(area.width),
                height: 1,
            });
        }
    }
}

fn draw_key_display(frame: &mut Frame, area: Rect, app: &App, _needs_image_redraw: bool) {
    let key_str = match &app.last_key {
        Some(k) => get_display_char(k),
        None => return,
    };

    // Use BigText widget for large, stylish key display
    let big_text = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(KEY_TEXT))
        .right_aligned()  // Align to right (close to dog)
        .lines(vec![key_str.into()])
        .build();

    // Position at right side of area, vertically centered
    let text_height = 4u16; // Quadrant pixel size = 4 rows for 8px font
    let key_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(text_height) / 2,
        width: area.width,
        height: text_height.min(area.height),
    };
    frame.render_widget(big_text, key_area);
}

/// Convert key string to a display-friendly character/string
fn get_display_char(key: &str) -> String {
    match key {
        "␣" => "_".to_string(),      // Space shown as underscore
        "⏎" => "<-".to_string(),     // Enter/Return
        "⌫" => "<".to_string(),      // Backspace
        "⇥" => "->".to_string(),     // Tab
        "⎋" => "X".to_string(),      // Escape
        "⌦" => ">".to_string(),      // Delete
        "⇧" => "^".to_string(),      // Shift
        "⌃" => "C".to_string(),      // Control
        "⌥" => "A".to_string(),      // Alt/Option
        "⌘" => "@".to_string(),      // Command
        "↑" => "^".to_string(),      // Up arrow
        "↓" => "v".to_string(),      // Down arrow
        "←" => "<".to_string(),      // Left arrow
        "→" => ">".to_string(),      // Right arrow
        s => s.chars().next().map(|c| c.to_string()).unwrap_or("?".to_string()),
    }
}

use crate::app::{AnimationState, App};
use crate::font::get_char_sprite;
use crate::iterm2;
use crate::key_spritesheet;
use crate::sprite;
use crate::spritesheet;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

// Color Palette
const WHITE: Color = Color::Rgb(255, 255, 255);
const CREAM: Color = Color::Rgb(255, 252, 245);
const TEXT_BLUE: Color = Color::Rgb(60, 120, 200);
const GRAY: Color = Color::Rgb(150, 150, 150);

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Calculate centered content area
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

    // Simple 50/50 layout: key on top, dog on bottom
    let chunks = Layout::vertical([
        Constraint::Ratio(1, 2), // Key display (top half)
        Constraint::Ratio(1, 2), // Dog sprite (bottom half)
    ])
    .split(content);

    // Draw components
    draw_key_display(frame, chunks[0], app);
    draw_dog(frame, chunks[1], app);
}

fn draw_dog(frame: &mut Frame, area: Rect, app: &App) {
    // Use sprite sheet if available and terminal supports inline images
    if spritesheet::is_loaded() && iterm2::supports_inline_images() {
        display_spritesheet_frame(area, app);
    } else {
        draw_dog_sprite(frame, area, app);
    }
}

fn display_spritesheet_frame(area: Rect, app: &App) {
    let frame_data = match app.animation_state {
        AnimationState::Typing => spritesheet::get_typing_frame(app.animation_frame),
        AnimationState::Idle | AnimationState::Typed => spritesheet::get_idle_frame(app.animation_frame),
    };

    if let Some(data) = frame_data {
        // Calculate center position for the sprite
        let sprite_width = 20u16;
        let sprite_height = 10u16;
        let col = area.x + (area.width.saturating_sub(sprite_width)) / 2;
        let row = area.y + (area.height.saturating_sub(sprite_height)) / 2;

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
    let sprite_height = sprite.len() as u16;
    let sprite_x = area.x + (area.width.saturating_sub(sprite_width)) / 2;
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

fn draw_key_display(frame: &mut Frame, area: Rect, app: &App) {
    // Use key sprite sheet if available and terminal supports inline images
    if key_spritesheet::is_loaded() && iterm2::supports_inline_images() {
        if let Some(key) = &app.last_key {
            display_key_sprite(area, key);
            return;
        }
    }
    // Fall back to pixel font
    draw_key_pixel_font(frame, area, app);
}

fn display_key_sprite(area: Rect, key: &str) {
    if let Some(data) = key_spritesheet::get_key_sprite(key) {
        // Calculate center position for the key sprite
        let sprite_width = 15u16;
        let sprite_height = 10u16;
        let col = area.x + (area.width.saturating_sub(sprite_width)) / 2;
        let row = area.y + (area.height.saturating_sub(sprite_height)) / 2;

        let _ = iterm2::display_image_at_position(
            data,
            row,
            col,
            Some(sprite_width as u32),
            Some(sprite_height as u32),
        );
    }
}

fn draw_key_pixel_font(frame: &mut Frame, area: Rect, app: &App) {
    let key_char = match &app.last_key {
        Some(k) => k.chars().next().unwrap_or('?'),
        None => '?',
    };

    let char_sprite = get_char_sprite(key_char);

    let box_width = 11u16;
    let box_height = 7u16;
    let box_x = area.x + (area.width.saturating_sub(box_width)) / 2;
    let box_y = area.y + (area.height.saturating_sub(box_height)) / 2;

    let is_active = app.last_key.is_some();
    let border_color = if is_active { WHITE } else { GRAY };
    let text_color = if is_active { TEXT_BLUE } else { GRAY };

    // Top border
    let top = format!("╭{}╮", "─".repeat(box_width as usize - 2));
    let top_para = Paragraph::new(top)
        .style(Style::default().fg(border_color));
    frame.render_widget(top_para, Rect {
        x: box_x,
        y: box_y,
        width: box_width,
        height: 1,
    });

    // Character rows
    for (i, row) in char_sprite.iter().enumerate() {
        let content = format!("│{}│", row);
        let para = Paragraph::new(content)
            .style(Style::default().fg(text_color).add_modifier(Modifier::BOLD));

        frame.render_widget(para, Rect {
            x: box_x,
            y: box_y + 1 + i as u16,
            width: box_width,
            height: 1,
        });
    }

    // Bottom border
    let bot = format!("╰{}╯", "─".repeat(box_width as usize - 2));
    let bot_para = Paragraph::new(bot)
        .style(Style::default().fg(border_color));
    frame.render_widget(bot_para, Rect {
        x: box_x,
        y: box_y + 6,
        width: box_width,
        height: 1,
    });
}

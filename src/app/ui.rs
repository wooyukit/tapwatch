use super::spritesheet;
use super::state::{AnimationState, App};
use super::terminal;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    Frame,
};
use std::time::Duration;
use tachyonfx::Shader;
use tui_big_text::{BigText, PixelSize};

// Playful/cute text color - soft and friendly
const TEXT_MAIN: Color = Color::Rgb(255, 182, 193);    // Soft pink

pub fn draw(frame: &mut Frame, app: &mut App, needs_image_redraw: bool, elapsed: Duration) {
    let area = frame.area();

    // Content dimensions
    let text_height = 4u16;
    let dog_height = 10u16;
    let total_content_height = text_height + dog_height;

    // Center content vertically
    let v_pad = area.height.saturating_sub(total_content_height) / 2;
    let content = Rect {
        x: area.x,
        y: area.y + v_pad,
        width: area.width,
        height: total_content_height.min(area.height),
    };

    // Vertical layout: text on top, dog below (both centered together)
    let chunks = Layout::vertical([
        Constraint::Length(text_height),  // Text display (top)
        Constraint::Length(dog_height),   // Dog sprite (bottom)
    ])
    .split(content);

    // Draw components
    // Draw dog first
    draw_dog(frame, chunks[1], app, needs_image_redraw);

    // Draw text if there's any typed text
    if !app.typed_text.is_empty() {
        let (text_area, new_char_area) = draw_text_display(frame, chunks[0], app);

        // Apply typing effect (coalesce) only to new character area
        if let Some(ref mut effect) = app.typing_effect {
            if !effect.done() {
                if let Some(char_area) = new_char_area {
                    effect.process(elapsed.into(), frame.buffer_mut(), char_area);
                }
            }
        }

        // Apply fade-out effect to full text area
        if let Some(ref mut effect) = app.fade_effect {
            if !effect.done() {
                effect.process(elapsed.into(), frame.buffer_mut(), text_area);
            }
        }
    }
}

fn draw_dog(_frame: &mut Frame, area: Rect, app: &App, needs_image_redraw: bool) {
    // Display PNG sprite if spritesheet is loaded
    // No fallback - if terminal doesn't support images, just show text only
    if spritesheet::is_loaded() && needs_image_redraw {
        display_spritesheet_frame(area, app);
    }
}

fn display_spritesheet_frame(area: Rect, app: &App) {
    let frame_data = match app.animation_state {
        AnimationState::Typing => spritesheet::get_typing_frame(app.typing_frame),
        AnimationState::Idle => spritesheet::get_idle_frame(app.idle_frame),
    };

    if let Some(data) = frame_data {
        // Center sprite in area
        let sprite_width = 20u16;
        let sprite_height = 10u16;
        let col = area.x + (area.width.saturating_sub(sprite_width)) / 2;
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

/// Returns (full_text_area, new_char_area)
fn draw_text_display(frame: &mut Frame, area: Rect, app: &App) -> (Rect, Option<Rect>) {
    if app.typed_text.is_empty() {
        return (area, None);
    }

    // Center text horizontally and use full height
    let text_height = 4u16; // Quadrant pixel size = 4 rows
    let text_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(text_height) / 2,
        width: area.width,
        height: text_height.min(area.height),
    };

    // Calculate how many characters fit (each char is ~4 columns in Quadrant mode)
    let char_width = 4u16;
    let max_chars = (area.width / char_width) as usize;

    // Count displayed characters
    let total_chars = app.typed_text.chars().count();
    let displayed_chars = total_chars.min(max_chars);

    // Check if text fits in the area
    let text_fits = total_chars <= max_chars || max_chars == 0;

    let text = if text_fits {
        // Text fits - center it
        BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .style(Style::default().fg(TEXT_MAIN))
            .centered()
            .lines(vec![app.typed_text.clone().into()])
            .build()
    } else {
        // Text too long - show rightmost characters, right-aligned
        let start = app.typed_text.len() - max_chars;
        let display_text = &app.typed_text[start..];
        BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .style(Style::default().fg(TEXT_MAIN))
            .right_aligned()
            .lines(vec![display_text.to_string().into()])
            .build()
    };

    frame.render_widget(text, text_area);

    // Calculate the area for just the new character(s)
    let new_char_area = if app.new_char_count > 0 && displayed_chars > 0 {
        let new_chars_width = (app.new_char_count as u16) * char_width;

        if text_fits {
            // Text is centered - new char is at the end of centered text
            let total_text_width = (displayed_chars as u16) * char_width;
            let text_start_x = text_area.x + (text_area.width.saturating_sub(total_text_width)) / 2;
            let new_char_x = text_start_x + total_text_width - new_chars_width;

            Some(Rect {
                x: new_char_x,
                y: text_area.y,
                width: new_chars_width,
                height: text_area.height,
            })
        } else {
            // Text is right-aligned - new char is at the right edge
            Some(Rect {
                x: text_area.x + text_area.width - new_chars_width,
                y: text_area.y,
                width: new_chars_width,
                height: text_area.height,
            })
        }
    } else {
        None
    };

    (text_area, new_char_area)
}

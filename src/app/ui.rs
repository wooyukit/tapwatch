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

    // Vertical layout: text on top, dog below
    let chunks = Layout::vertical([
        Constraint::Length(4),   // Text display (top)
        Constraint::Min(10),     // Dog sprite (bottom)
    ])
    .split(content);

    // Draw components
    // Draw dog first
    draw_dog(frame, chunks[1], app, needs_image_redraw);

    // Draw text if there's any typed text
    if !app.typed_text.is_empty() {
        let text_area = draw_text_display(frame, chunks[0], app);

        // Apply fade-out effect if active
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

fn draw_text_display(frame: &mut Frame, area: Rect, app: &App) -> Rect {
    if app.typed_text.is_empty() {
        return area;
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

    // Check if text fits in the area
    let text_fits = app.typed_text.len() <= max_chars || max_chars == 0;

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
    text_area
}

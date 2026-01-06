use std::time::{Duration, Instant};

/// Animation frame duration for typing effect
const TYPING_ANIMATION_DURATION: Duration = Duration::from_millis(150);

/// How long the "typing" state persists after a keypress
const TYPING_STATE_DURATION: Duration = Duration::from_millis(300);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Typing,
    Typed,
}

pub struct App {
    /// Current animation state
    pub animation_state: AnimationState,
    /// Current animation frame within state (for multi-frame animations)
    pub animation_frame: usize,
    /// Last key that was pressed
    pub last_key: Option<String>,
    /// Time of last keypress (for animation timing)
    pub last_keypress_time: Option<Instant>,
    /// Time of last animation frame change
    pub last_frame_time: Instant,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Scanline offset for animation effect
    pub scanline_offset: u16,
    /// Frame counter for effects
    pub frame_count: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            animation_state: AnimationState::Idle,
            animation_frame: 0,
            last_key: None,
            last_keypress_time: None,
            last_frame_time: Instant::now(),
            should_quit: false,
            scanline_offset: 0,
            frame_count: 0,
        }
    }

    /// Handle a key press event
    pub fn on_key(&mut self, key: String) {
        self.last_key = Some(key);
        self.last_keypress_time = Some(Instant::now());
        self.animation_state = AnimationState::Typing;
        self.animation_frame = 0;
    }

    /// Update animation state based on timing
    pub fn tick(&mut self) {
        let now = Instant::now();
        self.frame_count = self.frame_count.wrapping_add(1);

        // Update scanline animation
        if self.frame_count % 3 == 0 {
            self.scanline_offset = (self.scanline_offset + 1) % 20;
        }

        // Update animation frame
        if now.duration_since(self.last_frame_time) >= TYPING_ANIMATION_DURATION {
            self.last_frame_time = now;
            self.animation_frame = (self.animation_frame + 1) % 4;
        }

        // Check if we should return to idle state
        if let Some(keypress_time) = self.last_keypress_time {
            let elapsed = now.duration_since(keypress_time);

            if elapsed >= TYPING_STATE_DURATION {
                if self.animation_state == AnimationState::Typing {
                    self.animation_state = AnimationState::Typed;
                }
            }

            if elapsed >= TYPING_STATE_DURATION * 2 {
                self.animation_state = AnimationState::Idle;
            }
        }
    }

    /// Request app to quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

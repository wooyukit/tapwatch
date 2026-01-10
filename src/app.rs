use rand::Rng;
use std::time::{Duration, Instant};

/// Animation frame duration for typing effect (slower)
const TYPING_ANIMATION_DURATION: Duration = Duration::from_millis(250);

/// Animation frame duration for idle effect (very slow - 10 seconds per frame)
const IDLE_ANIMATION_DURATION: Duration = Duration::from_secs(10);

/// Number of typing frames in the sprite sheet (rows 3-4)
const TYPING_FRAME_COUNT: usize = 8;

/// Number of idle frames in the sprite sheet (rows 1-2)
const IDLE_FRAME_COUNT: usize = 8;

/// How long to continue typing animation after last keypress
const TYPING_LINGER_DURATION: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Typing,
}

pub struct App {
    /// Current animation state
    pub animation_state: AnimationState,
    /// Current animation frame for typing animation
    pub typing_frame: usize,
    /// Current animation frame for idle animation
    pub idle_frame: usize,
    /// Time of the last keypress (for typing animation duration)
    pub last_keypress_time: Instant,
    /// Last key that was pressed
    pub last_key: Option<String>,
    /// Time of last typing animation frame change
    pub last_typing_frame_time: Instant,
    /// Time of last idle animation frame change
    pub last_idle_frame_time: Instant,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Scanline offset for animation effect
    pub scanline_offset: u16,
    /// Frame counter for effects
    pub frame_count: u64,
    /// Track last rendered state to avoid unnecessary image redraws
    pub last_rendered_state: Option<AnimationState>,
    /// Track last rendered frame index
    pub last_rendered_frame: usize,
    /// Track last rendered key
    pub last_rendered_key: Option<String>,
    /// Track last terminal size for redraw on resize
    pub last_terminal_size: (u16, u16),
}

impl App {
    pub fn new() -> Self {
        Self {
            animation_state: AnimationState::Idle,
            typing_frame: 0,
            idle_frame: 0,
            last_keypress_time: Instant::now(),
            last_key: None,
            last_typing_frame_time: Instant::now(),
            last_idle_frame_time: Instant::now(),
            should_quit: false,
            scanline_offset: 0,
            frame_count: 0,
            last_rendered_state: None,
            last_rendered_frame: 0,
            last_rendered_key: None,
            last_terminal_size: (0, 0),
        }
    }

    /// Handle a key press event
    pub fn on_key(&mut self, key: String) {
        self.last_key = Some(key);
        self.last_keypress_time = Instant::now();

        // Start typing animation if not already typing
        if self.animation_state != AnimationState::Typing {
            self.animation_state = AnimationState::Typing;
            self.typing_frame = 0;
            self.last_typing_frame_time = Instant::now();
        }
    }

    /// Update animation state based on timing
    pub fn tick(&mut self) {
        let now = Instant::now();
        self.frame_count = self.frame_count.wrapping_add(1);

        // Update scanline animation
        if self.frame_count % 3 == 0 {
            self.scanline_offset = (self.scanline_offset + 1) % 20;
        }

        // Animate based on current state
        match self.animation_state {
            AnimationState::Typing => {
                // Advance typing animation frames
                if now.duration_since(self.last_typing_frame_time) >= TYPING_ANIMATION_DURATION {
                    self.last_typing_frame_time = now;
                    self.typing_frame = (self.typing_frame + 1) % TYPING_FRAME_COUNT;
                }

                // Check if we should transition to idle (3 seconds after last keypress)
                if now.duration_since(self.last_keypress_time) >= TYPING_LINGER_DURATION {
                    self.animation_state = AnimationState::Idle;
                    // Randomly select an idle frame for variety
                    self.idle_frame = rand::thread_rng().gen_range(0..IDLE_FRAME_COUNT);
                    self.last_idle_frame_time = now;
                }
            }
            AnimationState::Idle => {
                // Slow idle animation
                if now.duration_since(self.last_idle_frame_time) >= IDLE_ANIMATION_DURATION {
                    self.last_idle_frame_time = now;
                    self.idle_frame = (self.idle_frame + 1) % IDLE_FRAME_COUNT;
                }
            }
        }
    }

    /// Request app to quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Check if the visual state has changed since last render
    pub fn needs_image_redraw(&self, terminal_size: (u16, u16)) -> bool {
        // Redraw if terminal size changed (images would be in wrong position)
        if self.last_terminal_size != terminal_size {
            return true;
        }
        if self.last_rendered_state != Some(self.animation_state) {
            return true;
        }
        let current_frame = match self.animation_state {
            AnimationState::Idle => self.idle_frame,
            AnimationState::Typing => self.typing_frame,
        };
        if self.last_rendered_frame != current_frame {
            return true;
        }
        if self.last_rendered_key != self.last_key {
            return true;
        }
        false
    }

    /// Mark the current state as rendered
    pub fn mark_rendered(&mut self, terminal_size: (u16, u16)) {
        self.last_rendered_state = Some(self.animation_state);
        self.last_rendered_frame = match self.animation_state {
            AnimationState::Idle => self.idle_frame,
            AnimationState::Typing => self.typing_frame,
        };
        self.last_rendered_key = self.last_key.clone();
        self.last_terminal_size = terminal_size;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

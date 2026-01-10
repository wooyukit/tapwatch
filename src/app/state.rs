use rand::Rng;
use std::time::{Duration, Instant};
use tachyonfx::{fx, Effect, Interpolation, Shader};

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

/// Duration for fade-out effect when stopping typing
const FADE_OUT_DURATION: u32 = 800; // milliseconds

/// Duration for coalesce effect when typing (fast, snappy)
const TYPING_EFFECT_DURATION: u32 = 150; // milliseconds

/// Maximum length of accumulated text (generous limit to prevent memory issues)
const MAX_TEXT_LENGTH: usize = 100;

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
    /// Accumulated typed text (cleared on special keys or timeout)
    pub typed_text: String,
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
    /// Track last rendered text
    pub last_rendered_text: String,
    /// Track last terminal size for redraw on resize
    pub last_terminal_size: (u16, u16),
    /// Effect for fade-out animation
    pub fade_effect: Option<Effect>,
    /// Effect for typing animation (coalesce)
    pub typing_effect: Option<Effect>,
    /// Time of last frame for effect delta calculation
    pub last_frame_time: Instant,
    /// Whether current text is from a special key (should be cleared on next regular key)
    pub is_special_key_text: bool,
    /// Number of new characters added in the last keypress (for partial animation)
    pub new_char_count: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            animation_state: AnimationState::Idle,
            typing_frame: 0,
            idle_frame: 0,
            last_keypress_time: Instant::now(),
            typed_text: String::new(),
            last_typing_frame_time: Instant::now(),
            last_idle_frame_time: Instant::now(),
            should_quit: false,
            scanline_offset: 0,
            frame_count: 0,
            last_rendered_state: None,
            last_rendered_frame: 0,
            last_rendered_text: String::new(),
            last_terminal_size: (0, 0),
            fade_effect: None,
            typing_effect: None,
            last_frame_time: Instant::now(),
            is_special_key_text: false,
            new_char_count: 0,
        }
    }

    /// Check if a key should be ignored (modifier keys)
    fn is_ignored_key(key: &str) -> bool {
        matches!(key, "⇧" | "⌃" | "⌥" | "⌘")  // Shift, Ctrl, Alt, Cmd
    }

    /// Check if a key is a special key that should clear accumulated text
    fn is_special_key(key: &str) -> bool {
        matches!(
            key,
            "␣" | "⏎" | "⇥" | "⌫" | "⎋" | "⌦"
            | "↑" | "↓" | "←" | "→"
            | "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10" | "F11" | "F12"
        )
    }

    /// Get display representation for special keys
    fn get_special_key_display(key: &str) -> &str {
        match key {
            "␣" => "Space",
            "⏎" => "Enter",
            "⇥" => "Tab",
            "⌫" => "Back",
            "⎋" => "Esc",
            "⌦" => "Del",
            "↑" => "Up",
            "↓" => "Down",
            "←" => "Left",
            "→" => "Right",
            s => s,
        }
    }

    /// Handle a key press event
    pub fn on_key(&mut self, key: String) {
        // Ignore modifier keys entirely
        if Self::is_ignored_key(&key) {
            return;
        }

        self.last_keypress_time = Instant::now();

        // Clear any fade effect since we're typing again
        self.fade_effect = None;

        // Handle special keys vs regular keys
        if Self::is_special_key(&key) {
            // Replace text with special key display
            let display = Self::get_special_key_display(&key).to_string();
            self.new_char_count = display.chars().count(); // All chars are "new"
            self.typed_text = display;
            self.is_special_key_text = true;
        } else {
            // If previous text was from a special key, clear it first
            if self.is_special_key_text {
                self.typed_text.clear();
                self.is_special_key_text = false;
            }

            // Track how many new characters we're adding
            self.new_char_count = key.chars().count();

            // Append to accumulated text
            self.typed_text.push_str(&key);

            // Limit text length
            if self.typed_text.len() > MAX_TEXT_LENGTH {
                // Keep only the last MAX_TEXT_LENGTH characters
                let start = self.typed_text.len() - MAX_TEXT_LENGTH;
                self.typed_text = self.typed_text[start..].to_string();
            }
        }

        // Start typing animation if not already typing
        if self.animation_state != AnimationState::Typing {
            self.animation_state = AnimationState::Typing;
            self.typing_frame = 0;
            self.last_typing_frame_time = Instant::now();
        }

        // Trigger coalesce effect for each keypress (text materializes)
        self.typing_effect = Some(fx::coalesce((TYPING_EFFECT_DURATION, Interpolation::QuadOut)));
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

                    // Start dissolve effect for the text (characters disappear randomly)
                    self.fade_effect = Some(fx::dissolve((FADE_OUT_DURATION, Interpolation::QuadOut)));
                }
            }
            AnimationState::Idle => {
                // Slow idle animation
                if now.duration_since(self.last_idle_frame_time) >= IDLE_ANIMATION_DURATION {
                    self.last_idle_frame_time = now;
                    self.idle_frame = (self.idle_frame + 1) % IDLE_FRAME_COUNT;
                }

                // Clear text after fade effect completes
                if let Some(ref effect) = self.fade_effect {
                    if effect.done() {
                        self.typed_text.clear();
                        self.fade_effect = None;
                    }
                }
            }
        }
    }

    /// Request app to quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Get elapsed time since last frame and reset timer
    pub fn get_elapsed(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;
        elapsed
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
        if self.last_rendered_text != self.typed_text {
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
        self.last_rendered_text = self.typed_text.clone();
        self.last_terminal_size = terminal_size;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

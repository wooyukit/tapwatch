use ratatui::style::Color;

// Colors for the sprite
pub const DOG_WHITE: Color = Color::Rgb(255, 255, 255);
pub const DOG_CREAM: Color = Color::Rgb(255, 245, 230);
pub const DOG_LIGHT: Color = Color::Rgb(240, 230, 210);
pub const LAPTOP_GRAY: Color = Color::Rgb(200, 210, 220);
pub const LAPTOP_DARK: Color = Color::Rgb(150, 160, 170);
pub const SCREEN_BLUE: Color = Color::Rgb(180, 220, 255);
pub const DESK_BROWN: Color = Color::Rgb(180, 140, 100);

// Sprite sheet for dog at laptop
// Using block characters: █ ▀ ▄ ▌▐ ░ ▒ ▓ for pixel art
// Each frame is 20 chars wide x 12 rows tall

/// Dog sitting at laptop - idle frame (looking at screen)
pub const FRAME_IDLE: &[&str] = &[
    "                    ",
    "       ▄▄███▄▄      ",
    "      █▀     ▀█     ",
    "     █  ◠   ◠  █    ",  // Relaxed eyes
    "     █    ▼    █    ",  // Nose
    "      ▀▄▄▄▄▄▄▀      ",  // Mouth/chin
    "     ▄█████████▄    ",  // Body
    "    █░░░░░░░░░░░█   ",  // Arms on desk
    "    █░┌───────┐░█   ",  // Laptop screen
    "    █░│ ▪▪▪▪▪ │░█   ",  // Screen content
    "    █░└───────┘░█   ",  // Laptop base
    "   ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀  ",  // Desk
];

/// Dog typing - left paw up
pub const FRAME_TYPE_L: &[&str] = &[
    "        ♪           ",
    "       ▄▄███▄▄      ",
    "      █▀     ▀█     ",
    "     █  ●   ●  █    ",  // Alert eyes
    "     █    ▼    █    ",
    "      ▀▄▄▄▄▄▄▀      ",
    "     ▄█████████▄    ",
    "   ▄█░░░░░░░░░░░█   ",  // Left arm up
    "    █░┌───────┐░█   ",
    "    █░│ ▪▪▪▪▪ │░█   ",
    "    █░└───────┘░█   ",
    "   ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀  ",
];

/// Dog typing - right paw up
pub const FRAME_TYPE_R: &[&str] = &[
    "           ♫        ",
    "       ▄▄███▄▄      ",
    "      █▀     ▀█     ",
    "     █  ●   ●  █    ",  // Alert eyes
    "     █    ▼    █    ",
    "      ▀▄▄▄▄▄▄▀      ",
    "     ▄█████████▄    ",
    "    █░░░░░░░░░░░█▄  ",  // Right arm up
    "    █░┌───────┐░█   ",
    "    █░│ ▪▪▪▪▪ │░█   ",
    "    █░└───────┘░█   ",
    "   ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀  ",
];

/// Dog typing - both paws typing (middle frame)
pub const FRAME_TYPE_M: &[&str] = &[
    "      ♪   ♫         ",
    "       ▄▄███▄▄      ",
    "      █▀     ▀█     ",
    "     █  ◕   ◕  █    ",  // Happy eyes
    "     █    ▼    █    ",
    "      ▀▄▄‿▄▄▄▀      ",  // Smiling
    "     ▄█████████▄    ",
    "    █░░░░░░░░░░░█   ",
    "    █░┌───────┐░█   ",
    "    █░│▪▪▪▪▪▪▪│░█   ",  // More typing on screen
    "    █░└───────┘░█   ",
    "   ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀  ",
];

// Alternative cuter dog sprite (more cartoon style)
pub const CUTE_IDLE: &[&str] = &[
    "                    ",
    "      ╱▔▔▔▔▔╲       ",
    "     ╱ ˶ᵔ ᵕ ᵔ˶╲     ",  // Cute face
    "     ▏  ╰─╯   ▕     ",  // Nose/mouth
    "      ╲_____╱       ",
    "     ┌┴─────┴┐      ",  // Body
    "    ┌┘░░░░░░░└┐     ",
    "    │░┌─────┐░│     ",  // Laptop
    "    │░│ === │░│     ",
    "    │░│ === │░│     ",
    "    │░└─────┘░│     ",
    "    └─────────┘     ",
];

pub const CUTE_TYPE1: &[&str] = &[
    "         ♪          ",
    "      ╱▔▔▔▔▔╲       ",
    "     ╱ ˶◕ ᵕ ◕˶╲     ",  // Excited eyes
    "     ▏  ╰▽╯   ▕     ",
    "      ╲_____╱       ",
    "    ╱┌┴─────┴┐      ",  // Left arm up
    "    ┌┘░░░░░░░└┐     ",
    "    │░┌─────┐░│     ",
    "    │░│ =▪= │░│     ",
    "    │░│ === │░│     ",
    "    │░└─────┘░│     ",
    "    └─────────┘     ",
];

pub const CUTE_TYPE2: &[&str] = &[
    "          ♫         ",
    "      ╱▔▔▔▔▔╲       ",
    "     ╱ ˶◕ ᵕ ◕˶╲     ",
    "     ▏  ╰▽╯   ▕     ",
    "      ╲_____╱       ",
    "     ┌┴─────┴┐╲     ",  // Right arm up
    "    ┌┘░░░░░░░└┐     ",
    "    │░┌─────┐░│     ",
    "    │░│ =▪= │░│     ",
    "    │░│ =▪= │░│     ",
    "    │░└─────┘░│     ",
    "    └─────────┘     ",
];

// Simpler blocky sprite (retro pixel art style)
pub const PIXEL_IDLE: &[&str] = &[
    "      ▄████▄       ",
    "     ██░░░░██      ",
    "    ██ ▀  ▀ ██     ",
    "    ██  ▄▄  ██     ",
    "     ████████      ",
    "    ██████████     ",
    "   ██▒▒▒▒▒▒▒▒██    ",
    "   ██▒██████▒██    ",
    "   ██▒██▓▓██▒██    ",
    "   ██▒██████▒██    ",
    "   ██▀▀▀▀▀▀▀▀██    ",
];

pub const PIXEL_TYPE1: &[&str] = &[
    "      ▄████▄   ♪   ",
    "     ██░░░░██      ",
    "    ██ ●  ● ██     ",
    "    ██  ▄▄  ██     ",
    "     ████████      ",
    "   ▄██████████     ",
    "   ██▒▒▒▒▒▒▒▒██    ",
    "   ██▒██████▒██    ",
    "   ██▒██▓▓██▒██    ",
    "   ██▒██████▒██    ",
    "   ██▀▀▀▀▀▀▀▀██    ",
];

pub const PIXEL_TYPE2: &[&str] = &[
    "  ♫   ▄████▄       ",
    "     ██░░░░██      ",
    "    ██ ●  ● ██     ",
    "    ██  ▄▄  ██     ",
    "     ████████      ",
    "    ██████████▄    ",
    "   ██▒▒▒▒▒▒▒▒██    ",
    "   ██▒██████▒██    ",
    "   ██▒██▓▓██▒██    ",
    "   ██▒██████▒██    ",
    "   ██▀▀▀▀▀▀▀▀██    ",
];

/// Get animation frame based on state and frame number
pub fn get_frame(is_typing: bool, frame_num: usize) -> &'static [&'static str] {
    if is_typing {
        match frame_num % 3 {
            0 => PIXEL_TYPE1,
            1 => PIXEL_TYPE2,
            _ => PIXEL_TYPE1,
        }
    } else {
        PIXEL_IDLE
    }
}

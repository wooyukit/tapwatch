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

use rdev::Key;
use std::borrow::Cow;

/// Convert rdev Key to a display string (with shift support)
/// Returns Cow to avoid allocations for static strings
pub fn key_to_string(key: Key, shifted: bool) -> Cow<'static, str> {
    match key {
        // Letters (uppercase when shifted, lowercase otherwise)
        Key::KeyA => Cow::Borrowed(if shifted { "A" } else { "a" }),
        Key::KeyB => Cow::Borrowed(if shifted { "B" } else { "b" }),
        Key::KeyC => Cow::Borrowed(if shifted { "C" } else { "c" }),
        Key::KeyD => Cow::Borrowed(if shifted { "D" } else { "d" }),
        Key::KeyE => Cow::Borrowed(if shifted { "E" } else { "e" }),
        Key::KeyF => Cow::Borrowed(if shifted { "F" } else { "f" }),
        Key::KeyG => Cow::Borrowed(if shifted { "G" } else { "g" }),
        Key::KeyH => Cow::Borrowed(if shifted { "H" } else { "h" }),
        Key::KeyI => Cow::Borrowed(if shifted { "I" } else { "i" }),
        Key::KeyJ => Cow::Borrowed(if shifted { "J" } else { "j" }),
        Key::KeyK => Cow::Borrowed(if shifted { "K" } else { "k" }),
        Key::KeyL => Cow::Borrowed(if shifted { "L" } else { "l" }),
        Key::KeyM => Cow::Borrowed(if shifted { "M" } else { "m" }),
        Key::KeyN => Cow::Borrowed(if shifted { "N" } else { "n" }),
        Key::KeyO => Cow::Borrowed(if shifted { "O" } else { "o" }),
        Key::KeyP => Cow::Borrowed(if shifted { "P" } else { "p" }),
        Key::KeyQ => Cow::Borrowed(if shifted { "Q" } else { "q" }),
        Key::KeyR => Cow::Borrowed(if shifted { "R" } else { "r" }),
        Key::KeyS => Cow::Borrowed(if shifted { "S" } else { "s" }),
        Key::KeyT => Cow::Borrowed(if shifted { "T" } else { "t" }),
        Key::KeyU => Cow::Borrowed(if shifted { "U" } else { "u" }),
        Key::KeyV => Cow::Borrowed(if shifted { "V" } else { "v" }),
        Key::KeyW => Cow::Borrowed(if shifted { "W" } else { "w" }),
        Key::KeyX => Cow::Borrowed(if shifted { "X" } else { "x" }),
        Key::KeyY => Cow::Borrowed(if shifted { "Y" } else { "y" }),
        Key::KeyZ => Cow::Borrowed(if shifted { "Z" } else { "z" }),

        // Numbers and their shifted symbols (US keyboard layout)
        Key::Num0 => Cow::Borrowed(if shifted { ")" } else { "0" }),
        Key::Num1 => Cow::Borrowed(if shifted { "!" } else { "1" }),
        Key::Num2 => Cow::Borrowed(if shifted { "@" } else { "2" }),
        Key::Num3 => Cow::Borrowed(if shifted { "#" } else { "3" }),
        Key::Num4 => Cow::Borrowed(if shifted { "$" } else { "4" }),
        Key::Num5 => Cow::Borrowed(if shifted { "%" } else { "5" }),
        Key::Num6 => Cow::Borrowed(if shifted { "^" } else { "6" }),
        Key::Num7 => Cow::Borrowed(if shifted { "&" } else { "7" }),
        Key::Num8 => Cow::Borrowed(if shifted { "*" } else { "8" }),
        Key::Num9 => Cow::Borrowed(if shifted { "(" } else { "9" }),

        // Special keys
        Key::Space => Cow::Borrowed(" "),
        Key::Return => Cow::Borrowed("⏎"),
        Key::Tab => Cow::Borrowed("⇥"),
        Key::Backspace => Cow::Borrowed("⌫"),
        Key::Escape => Cow::Borrowed("⎋"),
        Key::Delete => Cow::Borrowed("⌦"),

        // Arrow keys
        Key::UpArrow => Cow::Borrowed("↑"),
        Key::DownArrow => Cow::Borrowed("↓"),
        Key::LeftArrow => Cow::Borrowed("←"),
        Key::RightArrow => Cow::Borrowed("→"),

        // Modifiers
        Key::ShiftLeft | Key::ShiftRight => Cow::Borrowed("⇧"),
        Key::ControlLeft | Key::ControlRight => Cow::Borrowed("⌃"),
        Key::Alt | Key::AltGr => Cow::Borrowed("⌥"),
        Key::MetaLeft | Key::MetaRight => Cow::Borrowed("⌘"),

        // Function keys
        Key::F1 => Cow::Borrowed("F1"),
        Key::F2 => Cow::Borrowed("F2"),
        Key::F3 => Cow::Borrowed("F3"),
        Key::F4 => Cow::Borrowed("F4"),
        Key::F5 => Cow::Borrowed("F5"),
        Key::F6 => Cow::Borrowed("F6"),
        Key::F7 => Cow::Borrowed("F7"),
        Key::F8 => Cow::Borrowed("F8"),
        Key::F9 => Cow::Borrowed("F9"),
        Key::F10 => Cow::Borrowed("F10"),
        Key::F11 => Cow::Borrowed("F11"),
        Key::F12 => Cow::Borrowed("F12"),

        // Punctuation and their shifted symbols
        Key::Comma => Cow::Borrowed(if shifted { "<" } else { "," }),
        Key::Dot => Cow::Borrowed(if shifted { ">" } else { "." }),
        Key::Slash => Cow::Borrowed(if shifted { "?" } else { "/" }),
        Key::SemiColon => Cow::Borrowed(if shifted { ":" } else { ";" }),
        Key::Quote => Cow::Borrowed(if shifted { "\"" } else { "'" }),
        Key::LeftBracket => Cow::Borrowed(if shifted { "{" } else { "[" }),
        Key::RightBracket => Cow::Borrowed(if shifted { "}" } else { "]" }),
        Key::BackSlash => Cow::Borrowed(if shifted { "|" } else { "\\" }),
        Key::Minus => Cow::Borrowed(if shifted { "_" } else { "-" }),
        Key::Equal => Cow::Borrowed(if shifted { "+" } else { "=" }),
        Key::BackQuote => Cow::Borrowed(if shifted { "~" } else { "`" }),

        // Unknown keys
        Key::Unknown(code) => Cow::Owned(format!("?{}", code)),
        _ => Cow::Borrowed("?"),
    }
}

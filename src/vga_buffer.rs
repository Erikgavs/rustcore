use std::ascii::Char::Backspace; // Backspace -> This import is unused; a no_std kernel should not depend on std.

#[allow(dead_code)] // allow(dead_code) -> Hide warnings while the colors are not used yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // derive -> Automatically implement debug formatting, copying, and equality.
#[repr(u8)] // repr(u8) -> Represent each Color variant as a u8 value.

// Color -> VGA palette names mapped to numbers (these are color IDs, not RGB values).
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // derive -> Automatically implement debug formatting, copying, and equality.
#[repr(transparent)] // repr(transparent) -> Store this type like the u8 it wraps.

struct ColorCode(u8); // ColorCode -> One byte holding both colors: background above, text below.

impl ColorCode { // impl -> Define functions associated with ColorCode.
    // Example call -> ColorCode::new(Color::White, Color::Blue) means white text on a blue background.
    // The caller chooses both colors and passes foreground first, then background.
    fn new(foreground: Color, background: Color) -> ColorCode { // new -> Build a color code from text and background colors.
        let foreground_code = foreground as u8; // as u8 -> Get the text color's numeric ID from the Color enum.
        let background_code = (background as u8) << 4; // << 4 -> Shift the background into the upper 4 bits (e.g. 4 becomes 64), leaving the lower 4 bits for the text color.
        ColorCode(background_code | foreground_code) // | -> Combine both color IDs into one byte; ColorCode(...) wraps it and returns it.
    }
}

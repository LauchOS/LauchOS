use crate::general::color::Color;

/// ScreenChar represents the printed char with a specific color. <br>
/// `ascii_character: u8`, <br>
/// `color_code: ColorCode`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

/// ColorCode represents a color, that is calculated by background-/ and foreground-colors. <br>
/// `ColorCode: u8`, <br>
/// <br>
/// `fn new(Color, Color) -> ColorCode` <br>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /// Calculates a new color with background-/ and foreground-color. <br>
    pub fn new(foreground_color: Color, background_color: Color) -> ColorCode {
        ColorCode((background_color as u8) << 4 | (foreground_color as u8))
    }
}
/// Color enum with hex values.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

impl Color{
    pub fn from_str(color_str: &str) -> Option<Color> {
        match color_str.to_ascii_lowercase().as_str() {
            "black" => Some(Color::Black),
            "blue" => Some(Color::Blue),
            "green" => Some(Color::Green),
            "cyan" => Some(Color::Cyan),
            "red" => Some(Color::Red),
            "magenta" => Some(Color::Magenta),
            "brown" => Some(Color::Brown),
            "lightgray" => Some(Color::LightGray),
            "darkgray" => Some(Color::DarkGray),
            "lightblue" => Some(Color::LightBlue),
            "lightgreen" => Some(Color::LightGreen),
            "lightcyan" => Some(Color::LightCyan),
            "lightred" => Some(Color::LightRed),
            "pink" => Some(Color::Pink),
            "yellow" => Some(Color::Yellow),
            "white" => Some(Color::White),
            _ => None,
        }
    }
}

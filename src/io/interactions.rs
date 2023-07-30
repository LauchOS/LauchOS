use crate::general::color::Color;
use crate::io::screen_writer::SCREENWRITER;
use crate::io::vga_buffer::screen_char::ColorCode;
use super::vga_buffer::BUFFER_HEIGHT;

/// Removes the last input character and adjusts cursor position.
pub fn remove_last_character_from_lowest_line(){
    SCREENWRITER.lock().remove_last_char_from_lowest_line();
}

/// Clears all rows of the `SCREEN_WRITER` buffer.
pub fn clear(){
    for i in 0..BUFFER_HEIGHT {
        SCREENWRITER.lock().clear_row(i);
    }
}

/// Sets the `screen_writer` foreground and background `Color_Code` based on <br>
/// `foreground: Color`, <br>
/// `background: Color`
pub fn change_color(foreground: Color, background: Color){
    SCREENWRITER.lock().change_color(ColorCode::new(foreground ,background));
}
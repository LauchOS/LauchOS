use crate::io::screen_writer::SCREENWRITER;
use crate::vga_buffer::BUFFER_HEIGHT;

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
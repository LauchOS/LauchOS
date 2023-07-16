use crate::vga_buffer::screen_char::*;
use crate::general::color::Color;
use crate::vga_buffer::vga_buffer::VGABuffer;
use crate::vga_buffer::*;

use core::fmt::*;
use lazy_static::lazy_static;
use spin::Mutex;

/// The screen writer, that changes the VGA-Buffer. <br>
/// `column_position: usize`, <br>
/// `color_code: ColorCode` <br>
/// `buffer: &'static mut VGABuffer`
pub struct ScreenWriter {
    column_position: usize,
    color_code: ColorCode,
    pub buffer: &'static mut VGABuffer,
}

impl ScreenWriter {
    /// Writes only one byte into the VGA-Buffer. `b'\n'` creates a new line.
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.buffer.screen_chars[BUFFER_HEIGHT - 1][self.column_position].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// Writes a whole string into the VGA-Buffer. It uses `fn write_byte()` for the operation.
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..= 0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Creates a new line.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.screen_chars[row][col].read();
                self.buffer.screen_chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a whole row.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.screen_chars[row][col].write(blank);
        }
    }
}

impl Write for ScreenWriter {
    /// Implements write_str to the screen writer.
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    /// Creates an static global writer instance, that is saved by mutex implementation.
    pub static ref SCREENWRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
    });
}
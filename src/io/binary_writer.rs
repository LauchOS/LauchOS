use crate::general::color::ColorCode;
use crate::general::color::Color;
use crate::vga_buffer::vga_buffer::VGABuffer;
use crate::vga_buffer::screen_char::ScreenChar;
use crate::vga_buffer::BUFFER_HEIGHT;
use crate::vga_buffer::BUFFER_WIDTH;

pub struct BinaryWriter {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut VGABuffer,
}

impl BinaryWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }

    fn new_line(&mut self) {/* TODO */}
}

pub fn print_something(){
    let mut writer = BinaryWriter {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("World!");
}
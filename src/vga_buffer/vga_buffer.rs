use super::screen_char::ScreenChar;
use super::BUFFER_WIDTH;
use super::BUFFER_HEIGHT;

#[repr(transparent)]
pub struct VGABuffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

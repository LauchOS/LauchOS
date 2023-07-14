use super::screen_char::ScreenChar;
use super::BUFFER_WIDTH;
use super::BUFFER_HEIGHT;
use volatile::Volatile;

#[repr(transparent)]
pub struct VGABuffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

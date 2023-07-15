use super::screen_char::ScreenChar;
use super::BUFFER_WIDTH;
use super::BUFFER_HEIGHT;
use volatile::Volatile;

/// VGA-Buffer, that is an array of screen chars and that represents the VGA-Hardware. <br>
/// `screen_chars: Array<ScreenChar>`
#[repr(transparent)]
pub struct VGABuffer {
    pub screen_chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

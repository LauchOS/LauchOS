use core::fmt::Arguments;
use super::screen_writer::SCREENWRITER;
use core::fmt::Write;

/// Print macro for printing on the screen.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::print::_print(format_args!($($arg)*)));
}

/// Println macro for printing on the screen (After every call: break).
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Basic print function (no macro) for printing a string on the screen.
#[doc(hidden)]
pub fn _print(args: Arguments) {
    SCREENWRITER.lock().write_fmt(args).unwrap();
}
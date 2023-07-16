use core::fmt::Arguments;
use super::screen_writer::SCREENWRITER;

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
    use core::fmt::Write;

    SCREENWRITER.lock().write_fmt(args).unwrap();
}



/**
 * 
 * Tests
 * 
 */

/// Call function `println!` without panicking
#[test_case]
fn test_println_01() {
    println!("test_println_01 output");
}

/// Call function `println!` many times
#[test_case]
fn test_println_02() {
    for _ in 0..200 {
        println!("test_println_02 output");
    }
}

/// Check output of function `println!`
#[test_case]
fn test_println_03() {
    use crate::vga_buffer::BUFFER_HEIGHT;

    let s = "test_println_03 output";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = SCREENWRITER.lock().buffer.screen_chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

/// Call function `print!` without panicking 
#[test_case]
fn test_print_01() {
    print!("test_print_01 output");
}

/// Call function `print!` many times
#[test_case]
fn test_print_02() {
    for _ in 0..200 {
        print!("test_print_02 output");
    }
}
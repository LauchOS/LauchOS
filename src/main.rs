#![feature(exclusive_range_pattern)]
#![no_std] // No rust std library
#![no_main] // Don't start with main function

use core::panic::PanicInfo;

mod vga_buffer;
mod general;
mod io;

// Start function (Linker looks for it)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World");

    loop {}
}

// Panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
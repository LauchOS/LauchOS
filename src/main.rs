#![feature(exclusive_range_pattern)]
#![no_std] // No rust std library
#![no_main] // Don't start with main function

use core::panic::PanicInfo;
use lauch_os::println;

/// Start-Function of the kernel. (Linker looks for it)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World");
    loop {}
}

/// Main Panic-Function for error handling.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
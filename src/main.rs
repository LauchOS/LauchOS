// Testing
#![test_runner(lauch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]

// Deactivates std library
#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Entry point for `cargo run`
#[no_mangle]
pub extern "C" fn _start() -> ! {
    lauch_os::println!("Hello World");

    lauch_os::init();

    // Call tests, if running test env.
    #[cfg(test)]
    test_main();

    loop {}
}

/// Panic function for error handling.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lauch_os::println!("{}", info);
    loop {}
}

/// Panic function for tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lauch_os::test_panic_handler(info)
}
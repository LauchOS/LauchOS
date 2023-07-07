#![no_std] // No rust std library
#![no_main] // Don't start with main function

use core::panic::PanicInfo;

// Start function (Linker looks for it)
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// Panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
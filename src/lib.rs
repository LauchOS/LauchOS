// Allocator
#![feature(const_mut_refs)]

// Interrupt
#![feature(abi_x86_interrupt)]

// Testing
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// No std library
#![no_std]

#[cfg(test)]
use bootloader::entry_point;

use bootloader::BootInfo;

#[cfg(test)]
entry_point!(test_kernel_main);

extern crate alloc;

pub mod io;
pub mod general;
pub mod vga_buffer;
pub mod qemu;
pub mod testing;
pub mod interrupt;
pub mod memory;
pub mod multitasking;

// Programs
mod shell;

use core::panic::PanicInfo;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Test helper for panic handling.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    use qemu::exit::*;

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();  
}

/// General function for init kernel.
pub fn init(boot_info: &'static BootInfo){
    interrupt::gdt::init_gdt();
    interrupt::interrupt::init_idt();
    unsafe { interrupt::pics::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    memory::allocator::init_allocators(boot_info);
    
    // Start shell (program)
    // shell::shell::init_shell();
}

/// General function for running tests.
pub fn test_runner(tests: &[&dyn testing::testable::Testable]) {
    use qemu::exit::*;
    
    serial_println!();
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    serial_println!();
    exit_qemu(QemuExitCode::Success);
}


/**
 * Test Enviorment
 */

/// Entry point for `cargo test --lib`
#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop();  
}

/// Panic function for integration tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

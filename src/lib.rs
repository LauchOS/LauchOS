// Allocator
#![feature(const_mut_refs)]

// Interrupt
#![feature(abi_x86_interrupt)]

// Testing
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

// No std library
#![no_std]

// Set test kernel entry point 
#[cfg(test)]
bootloader::entry_point!(test_kernel_main);

pub mod io;
pub mod general;
pub mod vga_buffer;
pub mod qemu;
pub mod testing;
pub mod interrupt;
pub mod memory;
pub mod multitasking;

// Will be changed by time
pub mod shell;

extern crate alloc;

/// General function for init kernel. <br>
/// Starting basic concepts for a working kernel.
pub fn init_kernel(boot_info: &'static bootloader::BootInfo){
    interrupt::gdt::init_gdt();
    interrupt::interrupt::init_idt();
    unsafe { interrupt::pics::PICS.lock().initialize() };

    x86_64::instructions::interrupts::enable();

    memory::allocator::init_allocators(boot_info);
}

/// Basic passiv waiting for interrupts. <br>
/// Assembler hlt function for holding CPU until the next interrupt.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}



/**
 * Testing for lib.rs
 */

/// Test kernel start
#[cfg(test)]
fn test_kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
    init_kernel(boot_info);
    test_main();    // Calls test_runner
    hlt_loop();
}

/// Panic function for tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    testing::test_panic_handler(info);
}

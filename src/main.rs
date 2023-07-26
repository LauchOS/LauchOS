// Testing
#![test_runner(lauch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]

// Deactivates std library
#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

extern crate alloc;
use alloc::boxed::Box;
use lauch_os::println;

entry_point!(kernel_main);

/// Entry point for `cargo run`
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use lauch_os::memory::allocator;
    use lauch_os::memory::allocator::BootInfoFrameAllocator;
    use lauch_os::memory::offset_page_table::init_opt;
    use x86_64::VirtAddr;

    lauch_os::init();

    // Frame Allocator
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_opt(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // Basic Allocator
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // Call tests, if running test env.
    #[cfg(test)]
    test_main();

    lauch_os::hlt_loop();
}

/// Panic function for error handling.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lauch_os::println!("{}", info);
    lauch_os::hlt_loop();
}

/// Panic function for tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lauch_os::test_panic_handler(info)
}
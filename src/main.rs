// Testing
#![test_runner(lauch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]

// Deactivates std library
#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use lauch_os::multitasking::{executor::Executor, tasks::{Task, keyboard::print_keypresses}};

entry_point!(kernel_main);

/// Entry point for `cargo run`
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    lauch_os::init();

    // Allocator
    init_allocator(boot_info);

    // Run two tasks
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(print_keypresses()));
    executor.run();

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

/// Init frame and basic allocator
fn init_allocator(boot_info: &'static BootInfo){
    use lauch_os::memory::allocator;
    use lauch_os::memory::allocator::frame_allocator::BootInfoFrameAllocator;
    use lauch_os::memory::offset_page_table::init_opt;
    use x86_64::VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_opt(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // Basic Allocator
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
}

/// Async function for example task
async fn async_number() -> u32 {
    42
}

/// Example Task
async fn example_task() {
    use lauch_os::println;
    let number = async_number().await;
    println!("async number: {}", number);
}
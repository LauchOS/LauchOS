// Testing
#![test_runner(lauch_os::general::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]

// Deactivates std library
#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

// Set main function (kernel entry point)
bootloader::entry_point!(kernel_main);

/// Entry point for kernel
fn kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
    lauch_os::init_kernel(boot_info);

    // Run two tasks (example)
    // let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.spawn(Task::new(print_keypresses()));
    // executor.run();

    // Call tests, if running test env.
    #[cfg(test)]
    test_main();

    lauch_os::hlt_loop();
}

/// Panic function for error handling.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lauch_os::println!("{}", info);
    lauch_os::hlt_loop();
}

/// Panic function for tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lauch_os::general::testing::test_panic_handler(info)
}

// /// Async function for example task
// async fn async_number() -> u32 {
//     42
// }

// /// Example Task
// async fn example_task() {
//     let number = async_number().await;
//     lauch_os::println!("async number: {}", number);
// }
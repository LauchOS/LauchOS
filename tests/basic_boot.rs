#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lauch_os::general::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

bootloader::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
    lauch_os::init_kernel(boot_info);
    test_main();
    lauch_os::hlt_loop()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lauch_os::general::testing::test_panic_handler(info);
}

/// Call function `println!` without panicking
#[test_case]
fn test_println_01() {
    lauch_os::println!("test_println_01 output");
}
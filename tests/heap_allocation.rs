#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lauch_os::general::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

bootloader::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
    lauch_os::init_kernel(boot_info);
    test_main();
    lauch_os::hlt_loop()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lauch_os::general::testing::test_panic_handler(info)
}

#[test_case]
fn test_allocation_01() {
    use alloc::boxed::Box;

    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn test_allocation_02() {
    use alloc::vec::Vec;

    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    use lauch_os::memory::HEAP_SIZE;
    use alloc::boxed::Box;
    
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
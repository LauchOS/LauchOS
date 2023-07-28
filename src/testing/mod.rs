pub mod testable;

/// General function for running tests. <br>
/// Will be called by test_main()
pub fn test_runner(tests: &[&dyn testable::Testable]) {
    crate::serial_println!();
    crate::serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    crate::serial_println!();
    crate::qemu::exit::exit_qemu(crate::qemu::exit::QemuExitCode::Success);
}

/// Test helper for panic handling.
pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    crate::serial_println!("[failed]\n");
    crate::serial_println!("Error: {}\n", info);
    crate::qemu::exit::exit_qemu(crate::qemu::exit::QemuExitCode::Failed);
    crate::hlt_loop();
}
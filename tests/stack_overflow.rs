#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

bootloader::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static bootloader::BootInfo) -> ! {
    lauch_os::serial_print!("stack_overflow::stack_overflow...\t");

    lauch_os::interrupt::gdt::init_gdt();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lauch_os::testing::test_panic_handler(info)
}

/// Causing stack overflow
#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}


/**
 * Own IDT and handler
 */
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static::lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(lauch_os::interrupt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    lauch_os::serial_println!("[ok]");
    lauch_os::qemu::exit::exit_qemu(lauch_os::qemu::exit::QemuExitCode::Success);
    loop {}
}
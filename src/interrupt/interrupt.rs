use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;
use super::DOUBLE_FAULT_IST_INDEX;

// Creating static IDT.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX); // new
        }
        idt
    };
}

/// Init IDT.
pub fn init_idt() {
    IDT.load();
}

/**
 * Handler functions
 */

/// Breakpoint Exception handler.
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}



/**
 * 
 * Tests
 * 
 */

#[test_case]
fn test_breakpoint_exception_01() {
    x86_64::instructions::interrupts::int3();
}
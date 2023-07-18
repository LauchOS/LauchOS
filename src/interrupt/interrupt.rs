use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

// Creating static IDT.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

/// Init IDT.
pub fn init_idt() {
    IDT.load();
}

/// Breakpoint Exception handler.
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
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
pub mod shell;
mod string;
mod command;
mod command_list;

/// Currently static length of the shell buffer.
pub const BUFFER_LENGTH: usize = 20; // static length for now
static mut BUFFER: [char; BUFFER_LENGTH] = ['\0'; BUFFER_LENGTH]; // static buffer length until allocs are possible
static mut POINTER: usize = 0;
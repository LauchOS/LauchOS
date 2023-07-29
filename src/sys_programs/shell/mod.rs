use alloc::vec::Vec;

pub mod shell;
mod command;
mod command_list;

static mut BUFFER: Vec<char> = Vec::new();
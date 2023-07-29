use alloc::string::String;
use alloc::vec::Vec;

/// Struct for any shell command. <br>
/// pub command: String <br>
/// execute: ExecuteFn
pub struct Command {
    pub command: String,
    execute: ExecuteFn,
}

impl Command {
    /// Initializes new instance of `COMMAND`, given a <br>
    /// - command: String <br>
    /// - execute: ExecuteFn
    pub fn new(command: String, execute: ExecuteFn) -> Self {
        Self {command, execute }
    }

    /// Executes the `execute` function with the given `args`.
    pub fn run(&self, args: &Vec<String>) {
        (self.execute)(args)
    }
}

/// Type for executable `COMMAND` functions.
pub type ExecuteFn = fn(&Vec<String>);



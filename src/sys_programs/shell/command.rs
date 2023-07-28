use super::BUFFER_LENGTH;

/// Struct for any shell command. <br>
/// pub command: [char; BUFFER_LENGTH] <br>
/// execute: ExecuteFn
pub struct Command {
    pub command: [char; BUFFER_LENGTH],
    execute: ExecuteFn,
}

impl Command {
    /// Initializes new instance of `COMMAND`, given a <br>
    /// - command: [char; BUFFER_LENGTH] <br>
    /// - execute: ExecuteFn
    pub fn new(command: [char; BUFFER_LENGTH], execute: ExecuteFn) -> Self {
        Self { command, execute }
    }

    /// Executes the `execute` function with the given `args`.
    pub fn run(&self, args: &[[char; BUFFER_LENGTH]; 10]) {
        (self.execute)(args)
    }

    /// Returns actual length of `command`, without empty characters.
    pub fn length(&self) -> usize {
        for i in 0..self.command.len() {
            if self.command[i] == '\0' {
                return i;
            }
        }
        self.command.len()
    }
}

/// Type for executable `COMMAND` functions.
pub type ExecuteFn = fn(&[[char; BUFFER_LENGTH]; 10]);

/// Manual implementation of the Copy trait, no use case but necessary.
impl Copy for Command {}

/// Manual implementation of the Clone trait, no use case but necessary.
impl Clone for Command {
    fn clone(&self) -> Self {
        *self
    }
}

use crate::shell::shell::BUFFER_LENGTH;

pub struct Command {
    pub command: [char; BUFFER_LENGTH],
    execute: ExecuteFn,
}

impl Command {
    pub fn new(command: [char; BUFFER_LENGTH], execute: ExecuteFn) -> Self {
        Self { command, execute }
    }

    pub fn run(&self, args: &[[char; BUFFER_LENGTH]; 10]) {
        (self.execute)(args)
    }

    // Returns length until first empty character
    pub fn length(&self) -> usize {
        for i in 0..self.command.len() {
            if self.command[i] == '\0' {
                return i;
            }
        }
        self.command.len()
    }
}

// Type for executable action
pub type ExecuteFn = fn(&[[char; BUFFER_LENGTH]; 10]);

// Manual implementation of the Copy trait
impl Copy for Command {}

// Manual implementation of the Clone trait
impl Clone for Command {
    fn clone(&self) -> Self {
        *self
    }
}

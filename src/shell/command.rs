use crate::shell::shell::BUFFER_LENGTH;

pub struct Command {
    pub command: [char; BUFFER_LENGTH],
    execute: ExecuteFn
}

impl Command {
    pub fn new(command: [char; BUFFER_LENGTH], execute: ExecuteFn) -> Self{
        Self{command, execute}
    }
    pub fn run(&self, args: &[[char; BUFFER_LENGTH]; 10]){
        (self.execute)(args)
    }
    pub fn length(&self) -> usize {
        for i in 0..self.command.len() {
            if self.command[i] == '\0' {
                return i;
            }
        }
        self.command.len()
    }
}

pub type ExecuteFn = fn(&[[char; BUFFER_LENGTH]; 10]);


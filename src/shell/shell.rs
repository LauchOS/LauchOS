use crate::{print, println};
use crate::shell::command::Command;
use crate::shell::command_list;
use crate::shell::command_list::COMMANDS;
use crate::shell::string;
use crate::testing::testable::Testable;

pub const BUFFER_LENGTH: usize = 20; // static length for now
static mut BUFFER: [char; BUFFER_LENGTH] = ['\0'; BUFFER_LENGTH]; // static buffer length until allocs are possible
static mut POINTER: usize = 0;

pub fn init_shell(){
    command_list::init_commands();
    print!("$ ");
}

// Processes Unicode type data, executes commands on enter
pub fn input_key(char: char){
    print!("{}", char);
    if char == '\n'{
        unsafe {
            fire_command();
        }
        return;
    }
    unsafe {
        BUFFER[POINTER] = char;
        POINTER += 1;

        if POINTER == BUFFER_LENGTH {
            // @TODO throw exception at buffer overflow?
            POINTER = 0;
        }
    }
}

unsafe fn fire_command(){
    let mut success = false;
    for i in 0..COMMANDS.len() {
        if let Some(cmd) = &COMMANDS[i]{
            if string::are_chars_equal(&cmd.command, &BUFFER){
                success = true;
                let mut arg_count = 0;
                let mut pos_in_arg = 0;
                let mut args = [['\0'; BUFFER_LENGTH]; 10];

                for j in cmd.length()..BUFFER_LENGTH {
                    if BUFFER[j] == ' ' {
                        if j != cmd.length() {
                            arg_count += 1;
                        }
                        continue;
                    }
                    if BUFFER[j] == '\0' {
                        break;
                    }

                    args[arg_count][pos_in_arg] = BUFFER[j];
                    pos_in_arg += 1;
                }

                cmd.run(&args);
                break;
            }
        }
    }

    if !success {
        println!("Command not found.")
    }

    for i in 0..BUFFER_LENGTH {
        BUFFER[i] = '\0';
    }
    POINTER = 0;
    print!("$ ");

}








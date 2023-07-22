use crate::{print, println};
use crate::shell::command_list;
use crate::shell::command_list::COMMANDS;
use crate::shell::string;

pub const BUFFER_LENGTH: usize = 20; // static length for now
static mut BUFFER: [char; BUFFER_LENGTH] = ['\0'; BUFFER_LENGTH]; // static buffer length until allocs are possible
static mut POINTER: usize = 0;

pub fn init_shell(){
    command_list::init_commands();
    print!("$ ");
}

// Processes Unicode type data, executes commands on enter
pub fn input_key(char: char){

    if !char.is_control() {
        print!("{}", char);

        unsafe {
            BUFFER[POINTER] = char;
            POINTER += 1;

            if POINTER == BUFFER_LENGTH {
                // @TODO throw exception at buffer overflow?
                POINTER = 0;
            }
        }
    }

    // handle control characters
    match char {
        '\n' => unsafe{ fire_command()}
        '\u{0008}' => unsafe{ backspace_pressed()}
        '\t' => unsafe{ tab_pressed()}
        _ => {}
    }


}

unsafe fn fire_command(){
    print!("\n");
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
        print!("Command '");
        for i in 0..POINTER {
            print!("{}", BUFFER[i]);
        }
        println!("' not found.")
    }

    for i in 0..BUFFER_LENGTH {
        BUFFER[i] = '\0';
    }
    POINTER = 0;
    print!("$ ");
}

unsafe fn backspace_pressed(){
    if POINTER == 0 {return;}
    POINTER -= 1;
    BUFFER[POINTER] = '\0';
    // @TODO remove character from screen
}

// autocomplete command if possible
unsafe fn tab_pressed(){
    let mut char_freq_count = [0; BUFFER_LENGTH];
    let mut complete_char = ['\0'; BUFFER_LENGTH];

    // Iterate through each command and count character occurrences
    for i in 0..COMMANDS.len() {
        if let Some(cmd) = &COMMANDS[i]{
            for (j, &cmd_char) in cmd.command.iter().enumerate() {
                if cmd_char == '\0' {
                    break;
                }
                if cmd_char == BUFFER[j] || BUFFER[j] == '\0' {
                    if cmd_char == complete_char[j] || (complete_char[j] == '\0' && char_freq_count[j] == 0){
                        complete_char[j] = cmd_char;
                        char_freq_count[j] += 1;
                    }else{
                        complete_char[j] = '\0';
                    }
                } else {
                    break;
                }
            }
        }
    }
    for i in 0..complete_char.len() {
        if complete_char[i] == '\0' {break}
        if i < POINTER {continue}
        print!("{}", complete_char[i]);
        BUFFER[POINTER] = complete_char[i];
        POINTER += 1;
    }
}











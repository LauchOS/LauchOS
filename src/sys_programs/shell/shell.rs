use alloc::sync::Arc;
use futures_util::StreamExt;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyCode, layouts, ScancodeSet1};
use spin::Mutex;
use crate::io::interactions;
use crate::multitasking::executor::Executor;
use crate::multitasking::scancode_stream::{SCANCODE_STREAM};
use crate::multitasking::task::Task;
use crate::print;
use super::command_list;
use super::command_list::COMMANDS;
use super::string;
use super::{BUFFER, BUFFER_LENGTH, POINTER};

/// Sets commands up, starts shell
pub fn init_shell(){
    command_list::init_commands();
    print!("$ ");
    let executor = Arc::new(Mutex::new(Executor::new()));
    executor.lock().spawn(Task::new(handle_keypresses()));
    executor.lock().run();
}

async fn handle_keypresses() {
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
                                     HandleControl::Ignore);
    while let Some(scancode) = SCANCODE_STREAM.lock().next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(char) => process_unicode(char),
                    DecodedKey::RawKey(key) => process_rawkey(key)
                }
            }
        }
    }
}

fn process_rawkey(key: KeyCode){
    // @TODO
}

/// Processes Unicode type data from keyboard input. <br>
/// Executes commands on enter. <br>
/// Handles control actions.
fn process_unicode(char: char){
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
        '\n' => unsafe{ execute()}
        '\u{0008}' => unsafe{ backspace_pressed()}
        '\t' => unsafe{ tab_pressed()}
        _ => {}
    }


}

/// Finds correct `COMMAND` instance and executes its `executableFn`.
unsafe fn execute(){
    crate::print!("\n");
    let mut success = false;
    for i in 0..COMMANDS.len() {
        if let Some(cmd) = &COMMANDS[i]{
            if string::are_chars_first_word_equal(&cmd.command, &BUFFER){
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
        crate::print!("Command '");
        for i in 0..POINTER {
            crate::print!("{}", BUFFER[i]);
        }
        crate::println!("' not found.")
    }

    for i in 0..BUFFER_LENGTH {
        BUFFER[i] = '\0';
    }
    POINTER = 0;
    crate::print!("$ ");
}

/// Removes last written character from shell `BUFFER` and `vga_buffer`.
unsafe fn backspace_pressed(){
    if POINTER == 0 {return;}
    POINTER -= 1;
    BUFFER[POINTER] = '\0';
    interactions::remove_last_character_from_lowest_line();
}

/// Finds possible `COMMAND`instances and completes the command in shell `BUFFER` and `vga_buffer` if possible.
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
        crate::print!("{}", complete_char[i]);
        BUFFER[POINTER] = complete_char[i];
        POINTER += 1;
    }
}











use alloc::string::String;
use alloc::vec::Vec;
use futures_util::StreamExt;
use pc_keyboard::*;
use crate::drivers::keyboard::add_listener;
use crate::io::interactions;
use crate::multitasking::scancode_stream::SCANCODE_STREAM;
use crate::print;
use super::command_list::{self, COMMANDS};
use super::BUFFER;

/// Sets commands up, starts shell
pub async fn start() {
    SCANCODE_STREAM.try_lock();
    command_list::init_commands();
    print!("$ ");
    add_listener(handle_input);
}

fn handle_input(key: DecodedKey) {
    match key {
        DecodedKey::Unicode(char) => process_unicode(char),
        DecodedKey::RawKey(key) => process_rawkey(key)
    }
}

fn process_rawkey(_key: KeyCode) {
    // @TODO
}

/// Processes Unicode type data from keyboard input. <br>
/// Executes commands on enter. <br>
/// Handles control actions.
fn process_unicode(char: char) {
    if !char.is_control() {
        print!("{}", char);
        unsafe {
            BUFFER.push(char);
        }
    }

    // handle control characters
    match char {
        '\n' => unsafe { execute() }
        '\u{0008}' => unsafe { backspace_pressed() }
        '\t' => unsafe { tab_pressed() }
        _ => {}
    }
}

/// Finds correct `COMMAND` instance and executes its `executableFn`.
unsafe fn execute() {
    print!("\n");
    let input: String = BUFFER.iter().collect();
    let mut success = false;
    for cmd in &COMMANDS {
        if input.starts_with(cmd.command.as_str()) {
            success = true;
            let args: Vec<String> = input.split(" ").skip(1).map(String::from).collect();
            cmd.run(&args);
            break;
        }
    }

    if !success {
        print!("Command '{}' not found\n", input);
    }

    BUFFER.clear();
    print!("$ ");
}

/// Removes last written character from shell `BUFFER` and `vga_buffer`.
unsafe fn backspace_pressed() {
    if BUFFER.len() > 0 {
        BUFFER.remove(BUFFER.len() - 1);
        interactions::remove_last_character_from_lowest_line();
    }
}

/// Finds possible `COMMAND`instances and completes the command in shell `BUFFER` and `vga_buffer` if possible.
unsafe fn tab_pressed() {
    let mut char_freq_count = [0; 20];
    let mut complete_char = ['\0'; 20];

    // Iterate through each command and count character occurrences
    for cmd in &COMMANDS {
        for (i, cmd_char) in cmd.command.chars().enumerate() {
            if cmd_char == '\0' {
                break;
            }
            if i >= BUFFER.len() || cmd_char == BUFFER[i] {
                if cmd_char == complete_char[i] || (complete_char[i] == '\0' && char_freq_count[i] == 0) {
                    complete_char[i] = cmd_char;
                    char_freq_count[i] += 1;
                } else {
                    complete_char[i] = '\0';
                }
            } else {
                break;
            }
        }
    }
    let len = BUFFER.len();
    for i in 0..complete_char.len() {
        if complete_char[i] == '\0' { break; }
        if i < len { continue; }
        print!("{}", complete_char[i]);
        BUFFER.push(complete_char[i]);
    }
}











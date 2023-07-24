use crate::{print, println};
use crate::io::interactions;
use crate::shell::command::{Command, ExecuteFn};
use crate::shell::shell::BUFFER_LENGTH;
use crate::shell::string;

pub static mut COMMANDS: [Option<Command>; 3] = [None; 3]; // length currently static @TODO once alloc possible

// register all base commands
pub fn init_commands(){
    let commands: &[(&str, ExecuteFn)] = &[
        ("echo", echo),
        ("exit", exit),
        ("clear", clear)
    ];

    unsafe {
        for (i, &(name, execute_fn)) in commands.iter().enumerate() {
            COMMANDS[i] = Some(Command::new(string::to_char_array(name), execute_fn));
        }
    }
}

fn echo(args: &[[char; BUFFER_LENGTH]; 10]){
    for j in 0..args.len() {
        if args[j][0] == '\0'{
            break;
        }
        for i in 0..args[j].len() {
            if args[0][i] == '\0'{
                break;
            }
            print!("{}", args[0][i]);
        }
    }
    print!("\n");
}

fn exit(_args: &[[char; BUFFER_LENGTH]; 10]){
    // @TODO exit application / shell
    println!("No way outta here...");
}

fn clear(_args: &[[char; BUFFER_LENGTH]; 10]){
    interactions::clear();
}




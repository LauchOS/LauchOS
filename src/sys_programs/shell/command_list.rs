use alloc::string::String;
use alloc::vec::Vec;
use crate::{print, println};
use crate::io::interactions;
use super::command::{Command, ExecuteFn};

pub static mut COMMANDS: Vec<Command> = Vec::new();

/// Registers all base commands for the shell.
pub fn init_commands(){
    let commands: &[(&str, ExecuteFn)] = &[
        ("echo", echo),
        ("exit", exit),
        ("clear", clear)
    ];

    unsafe {
        for (_i, &(name, execute_fn)) in commands.iter().enumerate() {
            COMMANDS.push(Command::new(String::from(name), execute_fn));
        }
    }
}

fn echo(args: &Vec<String>){
    for arg in args{
        print!(" {}", arg);
    }
    print!("\n");
}

fn exit(_args: &Vec<String>){
    // @TODO exit application / shell
    println!("No way outta here...");
}

fn clear(_args: &Vec<String>){
    interactions::clear();
}




use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::{print, println};
use crate::general::color::Color;
use crate::io::interactions;
use super::command::{Command, ExecuteFn};

pub static mut COMMANDS: Vec<Command> = Vec::new();

/// Registers all base commands for the shell.
pub fn init_commands(){
    let commands: &[(&str, ExecuteFn)] = &[
        ("echo", echo),
        ("exit", exit),
        ("clear", clear),
        ("set-color", set_color),
        ("list-colors", list_colors)
    ];

    unsafe {
        for (_i, &(name, execute_fn)) in commands.iter().enumerate() {
            COMMANDS.push(Command::new(String::from(name), execute_fn));
        }
    }
}

fn echo(args: &Vec<String>){

    for (i, arg) in args.iter().enumerate(){
        print!("{}", arg);
        if i < args.len() - 1 {print!(" ")}
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

fn set_color(args: &Vec<String>){
    if args.len() != 2{
        println!("Invalid format for 'set-color'. \n\
        Required format is 'set-color textcolor backcolor'.\n\
        Example: 'set-color white black'");
        return;
    }
    if let Some(text_color) = Color::from_str(&args[0]){
        if let Some(back_color) = Color::from_str(&args[1]){
            if text_color != back_color{
                interactions::change_color(text_color, back_color);
            }else{
                println!("Invalid colors. Text color and background color shall not be the same.")
            }
        }else{
            println!("Invalid background color. Get valid colors with 'list-colors'.")
        }
    }else{
        println!("Invalid text color. Get valid colors with 'list-colors'.")
    }
}

fn list_colors(_args: &Vec<String>){
    const COLORS: [&str; 16] = [
        "Black",
        "Blue",
        "Green",
        "Cyan",
        "Red",
        "Magenta",
        "Brown",
        "LightGray",
        "DarkGray",
        "LightBlue",
        "LightGreen",
        "LightCyan",
        "LightRed",
        "Pink",
        "Yellow",
        "White",
    ];

    for color in COLORS{
        println!("'{}'", color);
    }
}




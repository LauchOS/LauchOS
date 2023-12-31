use alloc::string::{String, ToString};
use alloc::vec::Vec;
use pc_keyboard::layouts;
use pc_keyboard::layouts::AnyLayout;
use crate::{print, println};
use crate::drivers;
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
        ("list-colors", list_colors),
        ("set-layout", set_layout),
        ("list-layouts", list_layouts),
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
        Example: 'set-color white black'.\n\
        Get valid colors with 'list-colors'.");
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

fn set_layout(args: &Vec<String>){
    if args.len() != 1{
        println!("Invalid format for 'set-layout'. \n\
        Required format is 'set-layout language'.\n\
        Example: 'set-layout DE'. \n\
        Get valid layouts with 'list-layouts'");
        return;
    }
    let layout_raw: Option<AnyLayout> = match args[0].to_string().to_lowercase().as_str() {
        "azerty" => Some(AnyLayout::Azerty(layouts::Azerty)),
        "colemak" => Some(AnyLayout::Colemak(layouts::Colemak)),
        "dvp" => Some(AnyLayout::DVP104Key(layouts::DVP104Key)),
        "de" => Some(AnyLayout::De105Key(layouts::De105Key)),
        "dvorak" => Some(AnyLayout::Dvorak104Key(layouts::Dvorak104Key)),
        "jis" => Some(AnyLayout::Jis109Key(layouts::Jis109Key)),
        "uk" => Some(AnyLayout::Uk105Key(layouts::Uk105Key)),
        "us" => Some(AnyLayout::Us104Key(layouts::Us104Key)),
        _ => None
    };

    if let Some(layout) = layout_raw{
        drivers::keyboard::set_keyboard_layout(layout);
        println!("New layout set successfully.")
    }else{
        println!("Invalid keyboard layout for 'set-layout'. Get valid layouts with 'list-layouts'");
    }
}

fn list_layouts(_args: &Vec<String>){
    const LAYOUTS: [&str; 8] = [
        "AZERTY",
        "COLEMAK",
        "DVP",
        "DE",
        "DVORAK",
        "JIS",
        "UK",
        "US"
    ];

    for layout in LAYOUTS{
        println!("'{}'", layout);
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




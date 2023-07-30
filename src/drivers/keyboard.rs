use alloc::vec::Vec;
use futures_util::StreamExt;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, layouts, ScancodeSet1};
use crate::multitasking::scancode_stream::SCANCODE_STREAM;
use crate::print;

static mut LISTENERS: Vec<KeyboardListenerFn> = Vec::new();

pub type KeyboardListenerFn = fn(key: DecodedKey);

pub async fn start() {
    handle_keypresses().await;
}

async fn handle_keypresses() {
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
                                     HandleControl::Ignore);
    while let Some(scancode) = SCANCODE_STREAM.lock().next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                unsafe {
                    for listener in &LISTENERS {
                        print!("--");
                        listener(key);
                    }
                }
            }
        }
    }
}

pub fn add_listener(func: KeyboardListenerFn) {
    unsafe {
        LISTENERS.push(func);
    }
}

pub fn remove_listener(func: KeyboardListenerFn) {
    unsafe {
        let search = LISTENERS.iter().position(|&x| x == func);
        match search {
            Some(index) => {
                LISTENERS.remove(index);
            }
            _ => {}
        }
    }
}
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use futures_util::StreamExt;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyCode, KeyState, layouts, Modifiers, ScancodeSet1};
use crate::multitasking::scancode_stream::SCANCODE_STREAM;

static mut LISTENERS: Vec<KeyboardListenerFn> = Vec::new();

pub type KeyboardListenerFn = fn(key: DecodedKey, modifiers: &Modifiers);

pub enum LayoutType{
    US,
    UK
}

impl LayoutType {
    pub fn get_layouts() -> Vec<String>{
        vec!(String::from("US"), String::from("UK"))
    }
}

pub async fn start() {
    handle_keypresses().await;
}

async fn handle_keypresses() {
    let mut keyboard_us = Keyboard::new(layouts::Us104Key, ScancodeSet1,
                                     HandleControl::Ignore);
    let mut keyboard_uk = Keyboard::new(layouts::Uk105Key, ScancodeSet1,
                                        HandleControl::Ignore);
    let mut modifiers = Modifiers{lshift: false, rshift: false, lctrl: false, rctrl: false, numlock: false, capslock: false, alt_gr: false };
    let mut current_layout: LayoutType = LayoutType::UK;


    while let Some(scancode) = SCANCODE_STREAM.lock().next().await {

        let key_event_res = match current_layout {
            LayoutType::US => keyboard_us.add_byte(scancode),
            LayoutType::UK => keyboard_uk.add_byte(scancode)
        };
        if let Ok(Some(key_event)) = key_event_res{
            set_modifier(&mut modifiers, key_event.state == KeyState::Down, key_event.code);

            if key_event.state == KeyState::Down {
                let key_res = match current_layout {
                    LayoutType::US => keyboard_us.process_keyevent(key_event),
                    LayoutType::UK => keyboard_uk.process_keyevent(key_event)
                };

                if let Some(key) = key_res {
                    unsafe {
                        for listener in &LISTENERS {
                            listener(key, &modifiers);
                        }
                    }
                }
            }
        }
    }
}

fn set_modifier(modifiers: &mut Modifiers, pressed: bool, code: KeyCode){
    match code {
        KeyCode::ControlLeft => modifiers.lctrl = pressed,
        KeyCode::ControlRight => modifiers.rctrl = pressed,
        KeyCode::AltLeft | KeyCode::AltRight => modifiers.alt_gr = pressed,
        KeyCode::ShiftLeft => modifiers.lshift = pressed,
        KeyCode::ShiftRight => modifiers.rshift = pressed,
        KeyCode::NumpadLock => modifiers.numlock = pressed,
        KeyCode::CapsLock => modifiers.capslock = pressed,
        _ => {}
    }
}

pub fn add_keyboard_listener(func: KeyboardListenerFn) {
    unsafe {
        LISTENERS.push(func);
    }
}

pub fn remove_keyboard_listener(func: KeyboardListenerFn) {
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

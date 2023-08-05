use alloc::vec::Vec;
use futures_util::StreamExt;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyCode, KeyState, layouts, Modifiers, ScancodeSet1};
use crate::multitasking::scancode_stream::SCANCODE_STREAM;

static mut LISTENERS: Vec<KeyboardListenerFn> = Vec::new();

pub type KeyboardListenerFn = fn(key: DecodedKey, modifiers: &Modifiers);

pub async fn start() {
    handle_keypresses().await;
}

async fn handle_keypresses() {
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
                                     HandleControl::Ignore);
    let mut modifiers = Modifiers{lshift: false, rshift: false, lctrl: false, rctrl: false, numlock: false, capslock: false, alt_gr: false };
    while let Some(scancode) = SCANCODE_STREAM.lock().next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            set_modifier(&mut modifiers, key_event.state == KeyState::Down, key_event.code);
            if key_event.state == KeyState::Down {
                if let Some(key) = keyboard.process_keyevent(key_event) {
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

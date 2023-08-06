use alloc::vec::Vec;
use futures_util::StreamExt;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyCode, KeyState, layouts, Modifiers, ScancodeSet1};
use pc_keyboard::layouts::AnyLayout;
use crate::multitasking::scancode_stream::SCANCODE_STREAM;

static mut LISTENERS: Vec<KeyboardListenerFn> = Vec::new();

pub type KeyboardListenerFn = fn(key: DecodedKey, modifiers: &Modifiers);

static mut KEYBOARD: Keyboard<AnyLayout, ScancodeSet1> = Keyboard::new(ScancodeSet1::new(), AnyLayout::De105Key(layouts::De105Key), HandleControl::MapLettersToUnicode);

pub async fn start() {
    unsafe {
        handle_keypresses().await;
    }
}

/// Handles scancode received from `SCANCODE_STREAM`, gives keycode and modifiers to listeners.
async unsafe fn handle_keypresses() {
    let mut modifiers = Modifiers{lshift: false, rshift: false, lctrl: false, rctrl: false, numlock: false, capslock: false, alt_gr: false, rctrl2: false };

    while let Some(scancode) = SCANCODE_STREAM.lock().next().await {
        if let Ok(Some(key_event)) = KEYBOARD.add_byte(scancode) {
            set_modifier(&mut modifiers, key_event.state == KeyState::Down, key_event.code);

                if let Some(key) = KEYBOARD.process_keyevent(key_event.clone()) {
                    if key_event.state == KeyState::Down || key_event.state == KeyState::SingleShot {
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

/// Sets modifiers according to keycode.
fn set_modifier(modifiers: &mut Modifiers, pressed: bool, code: KeyCode){
    match code {
        KeyCode::LControl => modifiers.lctrl = pressed,
        KeyCode::RControl => modifiers.rctrl = pressed,
        KeyCode::LAlt | KeyCode::RAlt2 => modifiers.alt_gr = pressed,
        KeyCode::LShift => modifiers.lshift = pressed,
        KeyCode::RShift => modifiers.rshift = pressed,
        KeyCode::NumpadLock => modifiers.numlock = pressed,
        KeyCode::CapsLock => modifiers.capslock = pressed,
        _ => {}
    }
}

/// Change layout of `KEYBOARD` to given `AnyLayout`.
pub fn set_keyboard_layout(layout: AnyLayout){
    unsafe {
        KEYBOARD = Keyboard::new(ScancodeSet1::new(), layout, HandleControl::Ignore);
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

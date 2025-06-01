use usbd_hid::descriptor;

use super::key_state::KeySM;
use crate::cc_engine::keycodes::*;

pub fn process_keyboard_report(result: &[KeySM]) -> descriptor::KeyboardReport {
    let mut report = descriptor::KeyboardReport::default();

    let mut index = 0;
    for key in result.iter() {
        // add the keycode to our keyboard report
        match key.get_keycode() {
            Some(keycode) => {
                match keycode {
                    Key::Mod(modifier) => report.modifier |= modifier as u8,
                    Key::Hid(keycode) => {
                        if index < 6 {
                            report.keycodes[index] = keycode as u8;
                            index += 1;
                        }
                    }
                    _ => (), // ignore all other keycodes
                }
            }
            None => {
                // do nothing for none
            }
        }
    }

    report
}

pub fn process_media_report(result: &[KeySM]) -> descriptor::MediaKeyboardReport {
    let mut report = descriptor::MediaKeyboardReport { usage_id: 0 };

    for key in result.iter() {
        // add the keycode to our keyboard report
        match key.get_keycode() {
            Some(keycode) => {
                match keycode {
                    Key::Media(id) => report.usage_id = id as u16,
                    _ => (), // ignore all other keycodes
                }
            }
            None => {
                // do nothing for none
            }
        }
    }

    report
}

pub fn process_control_report(result: &[KeySM]) -> descriptor::SystemControlReport {
    let mut report = descriptor::SystemControlReport { usage_id: 0 };

    for key in result.iter() {
        // add the keycode to our keyboard report
        match key.get_keycode() {
            Some(keycode) => {
                match keycode {
                    Key::Sys(id) => report.usage_id = id as u8,
                    _ => (), // ignore all other keycodes
                }
            }
            None => {
                // do nothing for none
            }
        }
    }

    report
}

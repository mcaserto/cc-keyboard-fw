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
                    Key::Shft(keycode) => {
                        if index < 6 {
                            // auto apply shift for this keycode
                            report.keycodes[index] = keycode as u8;
                            report.modifier |= Mod::Lshft as u8;
                            index += 1;
                        }
                    }
                    Key::Ctrl(keycode) => {
                        if index < 6 {
                            // auto apply shift for this keycode
                            report.keycodes[index] = keycode as u8;
                            report.modifier |= Mod::Lctrl as u8;
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

#[allow(dead_code)]
pub fn process_media_report(result: &[KeySM]) -> descriptor::MediaKeyboardReport {
    let mut report = descriptor::MediaKeyboardReport { usage_id: 0 };

    for key in result.iter() {
        // add the keycode to our keyboard report
        match key.get_keycode() {
            Some(Key::Media(id)) => {
                // process media key ( Probably not working, still in progress )
                report.usage_id = id as u16
            }
            None => {
                // do nothing for none
            }
            _ => {
                // do nothing for other keycodes
            }
        }
    }

    report
}

#[allow(dead_code)]
pub fn process_control_report(result: &[KeySM]) -> descriptor::SystemControlReport {
    let mut report = descriptor::SystemControlReport { usage_id: 0 };

    for key in result.iter() {
        // add the keycode to our keyboard report
        match key.get_keycode() {
            Some(Key::Sys(id)) => report.usage_id = id as u8,
            None => {
                // do nothing for none
            }
            _ => {
                // do nothing for other keycodes
            }
        }
    }

    report
}

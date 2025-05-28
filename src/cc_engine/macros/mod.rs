#[allow(dead_code)]
use crate::cc_engine::{keycodes, tasks::resources};
use usbd_hid::descriptor::KeyboardReport;

use super::keycodes::CCKeycode;

// description: sends a string as keyboard input
pub fn send_string(str: &str) {
    let mut report = KeyboardReport::default();
    for character in str.chars() {
        if character.is_uppercase() {
            report.modifier |= keycodes::CCModifier::CC_LEFT_SHFT as u8;
            let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);
        } else {
            report.modifier = 0;
            let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);
        }

        // ignoring send error for now
        report.keycodes[0] = (keycodes::CCKeycode::from(character)).into();
        let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);

        // hacky to allow sending multiples of a single chracter
        report.keycodes[0] = 0;
        let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);
    }

    report = KeyboardReport::default();
    let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);
}

// description: sends a single keycode
// todo! Keep track of key status (pressed, released) and pass to macro
pub fn send_keycode(code: &CCKeycode) {
    let mut report = KeyboardReport::default();
    report.keycodes[0] = (*code).into();
    let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);

    // cancel it
    report.keycodes[0] = 0;
    let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);
}

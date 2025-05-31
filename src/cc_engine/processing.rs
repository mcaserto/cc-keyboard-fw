use usbd_hid::descriptor;

use super::key::KeySM;
use super::keycodes::CCModifier;
use crate::cc_engine::keycodes::CCKeycode;

pub fn process_poll_result(result: &[KeySM]) -> descriptor::KeyboardReport {
    let mut report = descriptor::KeyboardReport::default();

    let mut index = 0;
    for key in result.iter() {
        // add the keycode to our keyboard report
        match key.get_keycode() {
            Some(keycode) => {
                match keycode {
                    CCKeycode::LCTRL => report.modifier |= CCModifier::L_CTRL as u8,
                    CCKeycode::LSHFT => report.modifier |= CCModifier::L_SHFT as u8,
                    CCKeycode::LALT => report.modifier |= CCModifier::L_ALT as u8,
                    CCKeycode::LGUI => report.modifier |= CCModifier::L_GUI as u8,
                    _ => {
                        // process the keycode
                        if index < 6 {
                            report.keycodes[index] = u8::from(keycode);
                            index += 1;
                        }
                    }
                }
            }
            None => {
                // do nothing for none
            }
        }
    }

    report
}

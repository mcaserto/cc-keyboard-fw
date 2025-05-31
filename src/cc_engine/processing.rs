use usbd_hid::descriptor;

use super::key_state::KeySM;
use crate::cc_engine::keycodes::*;

pub fn process_poll_result(result: &[KeySM]) -> descriptor::KeyboardReport {
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

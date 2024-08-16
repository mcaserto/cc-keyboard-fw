pub mod cc_definitions;
pub mod keymap;
pub mod matrix;

use cc_definitions::{CCKey::Key, CCKey::Lay, CCKey::Mod};
use matrix::ActiveSwitch;
use usbd_hid::descriptor::KeyboardReport;

// process the keys that are pressed and return a keyboard report
pub fn process_keys(switches: [Option<ActiveSwitch>; 6]) -> KeyboardReport {
    let mut report = KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0, 0, 0, 0, 0, 0],
    };
    let mut active_index = 0;

    for switch in switches {
        match switch {
            Some(value) => {
                // valid switch
                let action = keymap::KEYMAP[value.row][value.column];

                // based on the action,
                match action {
                    Key(val) => {
                        report.keycodes[active_index] = val as u8;
                        active_index += 1;
                    }
                    Mod(val) => report.modifier |= val as u8,
                    Lay(_val) => {} // currently not implemented
                }
            }
            None => {
                // empty, we can break
                break;
            }
        }
    }
    report
}

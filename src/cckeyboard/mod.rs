pub mod cc_definitions;
pub mod keymap;
pub mod matrix;

use cc_definitions::{CCKey::Key, CCKey::Lay, CCKey::Mod};
use matrix::ActiveSwitch;
use usbd_hid::descriptor::KeyboardReport;

// loops through switches and determines if a layer key is pressed
fn get_layer_value(switches: &[Option<ActiveSwitch>; 6]) -> usize {
    for switch in switches {
        match switch {
            Some(value) => {
                // valid switch
                let action = keymap::KEYMAP[0][value.row][value.column];
                match action {
                    Lay(layer) => {
                        // layer shift was pressed, return the shift value
                        if layer < keymap::LAYERS {
                            return layer;
                        } else {
                            return 0;
                        }
                    }
                    _ => {
                        // continue since this is not a layer shift key
                        continue;
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    0
}

// process the keys that are pressed and return a keyboard report
pub fn process_keys(switches: [Option<ActiveSwitch>; 6]) -> KeyboardReport {
    let mut report = KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0, 0, 0, 0, 0, 0],
    };
    let mut active_index = 0;

    // determine what keymap layer we should be using
    let layer = get_layer_value(&switches);

    for switch in switches {
        match switch {
            Some(value) => {
                // valid switch
                let action = keymap::KEYMAP[layer][value.row][value.column];

                // based on the action,
                match action {
                    Key(val) => {
                        match val {
                            cc_definitions::Code::Passthrough => {
                                // this is a passthrough, handle keypress on base layer
                                let new_action = keymap::KEYMAP[0][value.row][value.column];
                                match new_action {
                                    Key(val) => {
                                        // standard keycode
                                        report.keycodes[active_index] = val as u8;
                                        active_index += 1;
                                    }
                                    Mod(val) => report.modifier |= val as u8,
                                    Lay(_val) => {} // processed in the above function
                                }
                            }
                            _ => {
                                // standard keycode
                                report.keycodes[active_index] = val as u8;
                                active_index += 1;
                            }
                        }
                    }
                    Mod(val) => report.modifier |= val as u8,
                    Lay(_val) => {} // this is processed in the above function
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

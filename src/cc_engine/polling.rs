use crate::cc_engine::matrix;
use crate::keymap;
use usbd_hid::descriptor;

use crate::cc_engine::keycodes::CCKeycode;

pub fn process_poll_result(result: &mut matrix::PollResult) -> descriptor::KeyboardReport {
    let mut active_layer = 0;

    // convert the poll results into keycodes
    let keycode_count = result.get_num_keys();
    let mut keys = [matrix::Key { row: 0, column: 0 }; 10];
    let mut storage_index = 0;

    // loop through and store our keys but also look for any shifts
    for store_index in 0..keycode_count {
        let key = result.pop_key();
        let keymap_index = (keymap::COLUMNS * key.row) + key.column;
        let keycode = keymap::KEYMAP[active_layer][keymap_index];

        match keycode {
            CCKeycode::CC_LAY(commanded_layer) => {
                active_layer = commanded_layer;
            }
            CCKeycode::CC_NONE => {
                // don't add to our list
            }
            _ => {
                // process the key
                keys[store_index] = key;
                storage_index += 1;
            }
        }
    }

    // now that we have the keys we want, create a keyboard report from them
    let mut report = descriptor::KeyboardReport::default();
    let mut keycode_index = 0;

    for index in 0..storage_index {
        let key = keys[index];
        let keymap_index = (keymap::COLUMNS * key.row) + key.column;
        let keycode = keymap::KEYMAP[active_layer][keymap_index];

        match keycode {
            CCKeycode::CC_NONE => {
                // do nothing for now
            }
            CCKeycode::CC_LAY(_layer) => {
                // do nothing for now
            }
            CCKeycode::CC_CTRL => report.modifier |= u8::from(keycode),
            CCKeycode::CC_SHFT => report.modifier |= u8::from(keycode),
            CCKeycode::CC__ALT => report.modifier |= u8::from(keycode),
            _ => {
                report.keycodes[keycode_index] = u8::from(keycode);
                keycode_index += 1;
            }
        }
    }

    report
}

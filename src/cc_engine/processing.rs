use usbd_hid::descriptor;

use super::key::Key;
use super::keycodes::CCModifier;
use super::tasks::resources;
use crate::cc_engine::keycodes::CCKeycode;

// pub fn process_poll_result(result: &mut matrix::PollResult) -> descriptor::KeyboardReport {
//     let mut active_layer = 0;
//     // list of keys we will process
//     let mut storage_index = 0;
//     let mut keys_to_process = [matrix::Key { row: 0, column: 0 }; 10];

//     // convert the poll results into keycodes
//     let pressed_keys = result.get_pressed_keys();
//     let keycode_count = result.get_num_keys();

//     // loop through and store our keys but also look for any shifts
//     for key in pressed_keys.iter().take(keycode_count) {
//         let keymap_index = (usize::from(keymap::COLUMNS) * key.row) + key.column;
//         let keycode = keymap::KEYMAP[active_layer][keymap_index];

//         match keycode {
//             CCKeycode::LAYER(commanded_layer) => {
//                 active_layer = usize::from(commanded_layer);
//             }
//             CCKeycode::_______ => {
//                 // don't add to our list
//             }
//             _ => {
//                 // process this key
//                 keys_to_process[storage_index] = *key;
//                 storage_index += 1;
//             }
//         }
//     }

//     // now that we have the keys we want, create a keyboard report from them
//     let mut report = descriptor::KeyboardReport::default();
//     let mut keycode_index = 0;

//     for key in keys_to_process.iter_mut().take(storage_index) {
//         let keymap_index = (usize::from(keymap::COLUMNS) * key.row) + key.column;
//         let keycode = keymap::KEYMAP[active_layer][keymap_index];

//         match keycode {
//             CCKeycode::_______ => {
//                 // do nothing for now
//             }
//             CCKeycode::LAYER(_layer) => {
//                 // do nothing for now
//             }
//             CCKeycode::MACRO(callback) => {
//                 // call the macro
//                 callback();
//                 // TODO! super hacky debounce for now, add real debounce for all keys
//                 let start_time = Instant::now();
//                 while (Instant::now() - start_time).as_millis() <= 200 {}
//             }
//             CCKeycode::L__CTRL => report.modifier |= CCModifier::L_CTRL as u8,
//             CCKeycode::L_SHIFT => report.modifier |= CCModifier::L_SHFT as u8,
//             CCKeycode::L___ALT => report.modifier |= CCModifier::L_ALT as u8,
//             CCKeycode::L___GUI => report.modifier |= CCModifier::L_GUI as u8,
//             CCKeycode::PASSTHR => {
//                 if active_layer > 0 {
//                     let keycode = keymap::KEYMAP[active_layer - 1][keymap_index];
//                     report.keycodes[keycode_index] = u8::from(keycode);
//                     keycode_index += 1;
//                 }
//             }
//             _ => {
//                 report.keycodes[keycode_index] = u8::from(keycode);
//                 keycode_index += 1;
//             }
//         }
//     }

//     report
// }

pub fn process_poll_result(result: &[Key]) -> descriptor::KeyboardReport {
    let mut report = descriptor::KeyboardReport::default();

    let mut index = 0;
    for key in result.iter() {
        // add the keycode to our keyboard report
        match key.get_keycode() {
            Some(keycode) => {
                match keycode {
                    CCKeycode::L__CTRL => report.modifier |= CCModifier::L_CTRL as u8,
                    CCKeycode::L_SHIFT => report.modifier |= CCModifier::L_SHFT as u8,
                    CCKeycode::L___ALT => report.modifier |= CCModifier::L_ALT as u8,
                    CCKeycode::L___GUI => report.modifier |= CCModifier::L_GUI as u8,
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

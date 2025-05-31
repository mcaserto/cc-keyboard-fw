use embassy_time::Instant;

use super::key::{self, KeyInfo, KeyState};
use crate::cc_engine::keycodes::CCKeycode;
use crate::config;

pub struct RegularKey {
    info: KeyInfo,
}

impl key::KeyboardKey for RegularKey {
    fn new(row: u8, column: u8) -> Self {
        Self {
            info: KeyInfo::new(row, column),
        }
    }

    fn process(&mut self, pressed: &bool, layer: &mut usize) {
        let previous_pressed = self.info.pressed;
        self.info.pressed = if *pressed {
            KeyState::Pressed
        } else {
            KeyState::Released
        };

        if previous_pressed != self.info.pressed && self.info.pressed == KeyState::Pressed {
            // transitioning from released to pressed
            self.info.timestamp_pressed = Instant::now();
            let keymap_index = (config::COLUMNS * self.info.row) + self.info.column;

            let keycode = config::KEYMAP[usize::from(*layer)][usize::from(keymap_index)];
            self.info.keycode = match keycode {
                CCKeycode::_______ => None,
                CCKeycode::LAYER(commanded_layer) => {
                    // set the layer
                    *layer = usize::from(commanded_layer);
                    Some(CCKeycode::LAYER(commanded_layer))
                }
                CCKeycode::PASSTHR => {
                    if *layer > 0 {
                        let keycode =
                            config::KEYMAP[usize::from(*layer - 1)][usize::from(keymap_index)];
                        Some(keycode)
                    } else {
                        None
                    }
                }
                CCKeycode::MACRO(callback) => {
                    // call the macro
                    callback();
                    None
                }
                CCKeycode::MT(modifier, keycode) => {
                    // modtap support
                    Some(CCKeycode::MT(modifier, keycode))
                }
                _ => Some(keycode),
            };
        } else if previous_pressed != self.info.pressed && self.info.pressed == KeyState::Released {
            // transitioning from pressed to released
            self.info.timestamp_released = Instant::now();

            // do keycode specific actions
            if let Some(keycode) = self.info.keycode {
                match keycode {
                    CCKeycode::LAYER(_commanded_layer) => {
                        // return our layer to the base layer
                        *layer = 0;
                    }
                    _ => (),
                }
            }

            self.info.keycode = None
        }
    }

    fn get_keycode(&self) -> Option<CCKeycode> {
        if let Some(keycode) = self.info.keycode {
            match keycode {
                // filtering out custom keycodes that should never be sent to a computer
                CCKeycode::LAYER(_commanded_layer) => None,
                CCKeycode::PASSTHR => None,
                CCKeycode::MACRO(_callback) => None,
                CCKeycode::MT(_mod, _keycode) => None,
                _ => self.info.keycode,
            }
        } else {
            None
        }
    }
}

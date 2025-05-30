// embassy
use embassy_time::Instant;

// crate
use super::keycodes::{CCKeycode, CCModifier};
use crate::keymap;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Clone, Copy)]
pub struct Key {
    pressed: KeyState,
    keycode: Option<CCKeycode>,
    row: u8,
    column: u8,
    timestamp_pressed: Instant,
    timestamp_released: Instant,
}

impl Key {
    // Description: Create a new key struct
    // Param: row The row in the keymap that this key is associated
    // Param: column The column in the keymap that this key is associated
    pub fn new(row: u8, column: u8) -> Self {
        Self {
            pressed: KeyState::Released,
            keycode: None,
            row: row,
            column: column,
            timestamp_pressed: Instant::MIN,
            timestamp_released: Instant::MIN,
        }
    }

    // Description: Processes a result from polling the matrix on this key
    // Param: pressed Resultant pressed state from polling
    pub fn process(&mut self, pressed: &bool, layer: &mut usize) {
        let previous_pressed = self.pressed;
        self.pressed = if *pressed {
            KeyState::Pressed
        } else {
            KeyState::Released
        };

        if previous_pressed != self.pressed && self.pressed == KeyState::Pressed {
            // transitioning from released to pressed
            self.timestamp_pressed = Instant::now();
            let keymap_index = (keymap::COLUMNS * self.row) + self.column;

            let keycode = keymap::KEYMAP[usize::from(*layer)][usize::from(keymap_index)];
            self.keycode = match keycode {
                CCKeycode::_______ => None,
                CCKeycode::LAYER(commanded_layer) => {
                    // set the layer
                    *layer = usize::from(commanded_layer);
                    None
                }
                CCKeycode::PASSTHR => {
                    if *layer > 0 {
                        let keycode =
                            keymap::KEYMAP[usize::from(*layer - 1)][usize::from(keymap_index)];
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
                CCKeycode::MOD(modifier, keycode) => {
                    // modtap support
                    if (Instant::now() - self.timestamp_pressed).as_millis() >= 200 {
                        // act as modifier
                        match modifier {
                            CCModifier::L_CTRL => Some(CCKeycode::L__CTRL),
                            CCModifier::L_SHFT => Some(CCKeycode::L_SHIFT),
                            CCModifier::L_ALT => Some(CCKeycode::L___ALT),
                            CCModifier::L_GUI => Some(CCKeycode::L___GUI),
                            _ => Some(CCKeycode::_______),
                        }
                    } else {
                        // act as normal keycode
                        Some(CCKeycode::from(keycode))
                    }
                }
                _ => Some(keycode),
            };
        } else {
            // transitioning from pressed to released
            self.timestamp_released = Instant::now();
            self.keycode = None;

            // do keycode specific actions
        }
    }

    pub fn get_keycode(&self) -> &Option<CCKeycode> {
        &self.keycode
    }
}

// embassy
use embassy_time::Instant;

// crate
use super::keycodes::{CCKeycode, CCModifier};
use crate::config;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    Tapped,
    Held,
    Released,
    Idle,
}

#[derive(Clone, Copy)]
pub struct Key {
    state: KeyState,
    keycode: CCKeycode,
    base_key: CCKeycode,
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
        let keymap_index = (config::COLUMNS * row) + column;
        Self {
            state: KeyState::Released,
            keycode: CCKeycode::_______,
            base_key: config::KEYMAP[0][usize::from(keymap_index)],
            row: row,
            column: column,
            timestamp_pressed: Instant::MIN,
            timestamp_released: Instant::MIN,
        }
    }

    fn tapped(&mut self, layer: &mut usize) {
        self.state = match self.state {
            KeyState::Tapped => KeyState::Held,
            KeyState::Held => KeyState::Held,
            KeyState::Released => KeyState::Tapped,
            KeyState::Idle => KeyState::Tapped,
        };
        let keymap_index = (config::COLUMNS * self.row) + self.column;
        self.keycode = config::KEYMAP[usize::from(*layer)][usize::from(keymap_index)];

        // pressed logic
        self.timestamp_pressed = Instant::now();
        let keymap_index = (config::COLUMNS * self.row) + self.column;

        self.keycode = match self.base_key {
            CCKeycode::_______ => CCKeycode::_______,
            CCKeycode::LAYER(commanded_layer) => {
                // set the layer
                *layer = usize::from(commanded_layer);
                CCKeycode::_______
            }
            CCKeycode::PASSTHR => {
                if *layer > 0 {
                    config::KEYMAP[usize::from(*layer - 1)][usize::from(keymap_index)]
                } else {
                    config::KEYMAP[usize::from(*layer)][usize::from(keymap_index)]
                }
            }
            CCKeycode::MACRO(callback) => {
                // call the macro
                callback();
                CCKeycode::_______
            }
            CCKeycode::MT(modifier, keycode) => {
                // modtap support
                CCKeycode::MT(modifier, keycode)
            }
            _ => self.base_key,
        };
    }

    fn held(&mut self, layer: &mut usize) {}

    fn released(&mut self, layer: &mut usize) {
        self.timestamp_released = Instant::now();
        self.keycode = CCKeycode::_______;

        // process
        match self.base_key {
            CCKeycode::LAYER(_commanded_layer) => {
                // return our layer to the base layer
                *layer = 0;
            }
            CCKeycode::MT(_modifier, keycode) => {
                if (self.timestamp_pressed - self.timestamp_released).as_millis() < 200 {
                    // press the key
                    self.keycode = CCKeycode::from(keycode);
                }
            }
            _ => (),
        }
    }

    // Description: Processes a result from polling the matrix on this key
    // Param: pressed Resultant pressed state from polling
    pub fn process(&mut self, pressed: &bool, layer: &mut usize) {
        // update state machine
        if *pressed {
            self.state = match self.state {
                KeyState::Tapped => KeyState::Held,
                KeyState::Held => KeyState::Held,
                KeyState::Released => KeyState::Tapped,
                KeyState::Idle => KeyState::Tapped,
            };
        } else {
            self.state = match self.state {
                KeyState::Tapped => KeyState::Released,
                KeyState::Held => KeyState::Released,
                KeyState::Released => KeyState::Idle,
                KeyState::Idle => KeyState::Idle,
            };
        }

        match self.state {
            KeyState::Idle => (),
            KeyState::Tapped => self.tapped(layer),
            KeyState::Held => self.held(layer),
            KeyState::Released => self.released(layer),
        }
    }

    pub fn get_keycode(&self) -> Option<CCKeycode> {
        match self.keycode {
            // filtering out custom keycodes that should never be sent to a computer
            CCKeycode::_______ => None,
            CCKeycode::LAYER(_commanded_layer) => None,
            CCKeycode::PASSTHR => None,
            CCKeycode::MACRO(_callback) => None,
            CCKeycode::MT(_mod, _keycode) => None,
            _ => Some(self.keycode),
        }
    }
}

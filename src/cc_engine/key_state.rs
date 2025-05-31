// embassy
use embassy_time::Instant;
use smart_leds::RGB8;

// crate
use super::{keycodes::CCKeycode, tasks::resources};
use crate::config;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    Tapped,
    Held,
    Released,
    Idle,
}

#[derive(Clone, Copy)]
pub struct KeySM {
    state: KeyState,
    active_key: CCKeycode,
    base_key: CCKeycode,
    row: u8,
    column: u8,
    timestamp_tapped: Instant,
    timestamp_released: Instant,
}

impl KeySM {
    // Description: Create a new key struct
    // Param: row The row in the keymap that this key is associated
    // Param: column The column in the keymap that this key is associated
    pub fn new(row: u8, column: u8) -> Self {
        Self {
            state: KeyState::Released,
            active_key: CCKeycode::_______,
            base_key: CCKeycode::_______,
            row: row,
            column: column,
            timestamp_tapped: Instant::MIN,
            timestamp_released: Instant::MIN,
        }
    }

    fn tapped(&mut self, layer: &mut usize) {
        // initializer tapped state
        let keymap_index = (config::COLUMNS * self.row) + self.column;
        self.active_key = config::KEYMAP[usize::from(*layer)][usize::from(keymap_index)];
        self.timestamp_tapped = Instant::now();

        match self.active_key {
            CCKeycode::LAYER(commanded_layer) => {
                // store base keycode
                resources::STATUS_SIGNAL.signal(resources::Status::Color(RGB8::new(0, 0, 50)));
                *layer = usize::from(commanded_layer);
            }
            CCKeycode::PASSTHR => {
                if *layer > 0 {
                    self.active_key =
                        config::KEYMAP[usize::from(*layer - 1)][usize::from(keymap_index)]
                } else {
                    self.active_key = config::KEYMAP[usize::from(*layer)][usize::from(keymap_index)]
                }
            }
            CCKeycode::MACRO(callback) => {
                // call the macro
                callback();
            }
            CCKeycode::MT(_modifier, _keycode) => {
                // do nothing on a tap for a MT key
            }
            _ => (),
        };
    }

    fn held(&mut self) {
        match self.active_key {
            CCKeycode::MT(_tap_key, hold_key) => {
                self.base_key = self.active_key;

                if (Instant::now() - self.timestamp_tapped).as_millis() >= config::MOD_TAP_THRESHOLD
                {
                    self.active_key = CCKeycode::from(hold_key);
                }
            }
            _ => (),
        }
    }

    fn released(&mut self, layer: &mut usize) {
        self.timestamp_released = Instant::now();

        // process
        match self.active_key {
            CCKeycode::LAYER(_commanded_layer) => {
                // return our layer to the base layer
                *layer = 0;
                self.active_key = CCKeycode::_______;
                resources::STATUS_SIGNAL.signal(resources::Status::Heartbeat);
            }
            CCKeycode::MT(tap_key, _hold_key) => {
                if (self.timestamp_released - self.timestamp_tapped).as_millis()
                    < config::MOD_TAP_THRESHOLD
                {
                    self.active_key = CCKeycode::from(tap_key)
                } else {
                    self.active_key = self.base_key;
                }
            }
            _ => self.active_key = CCKeycode::_______,
        }
    }

    fn idle(&mut self) {
        // during idle, check if our base key is MT, and reset back to the base
        match self.base_key {
            CCKeycode::MT(_tap_key, _hold_key) => {
                self.active_key = self.base_key;
                self.base_key = CCKeycode::_______;
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

        // act on current state
        match self.state {
            KeyState::Idle => self.idle(),
            KeyState::Tapped => self.tapped(layer),
            KeyState::Held => self.held(),
            KeyState::Released => self.released(layer),
        }
    }

    pub fn get_keycode(&self) -> Option<CCKeycode> {
        match self.active_key {
            // filtering out custom keycodes that should never be sent to a computer
            CCKeycode::_______ => None,
            CCKeycode::LAYER(_commanded_layer) => None,
            CCKeycode::PASSTHR => None,
            CCKeycode::MACRO(_callback) => None,
            CCKeycode::MT(_mod, _keycode) => None,
            _ => Some(self.active_key),
        }
    }
}

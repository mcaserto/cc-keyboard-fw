// embassy
use embassy_time::Instant;
use smart_leds::RGB8;

// crate
use super::{
    keycodes::{Hid, Key},
    tasks::resources,
};
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
    active_key: Key,
    base_key: Key,
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
            active_key: Key::Hid(Hid::Nop),
            base_key: Key::Hid(Hid::Nop),
            row,
            column,
            timestamp_tapped: Instant::MIN,
            timestamp_released: Instant::MIN,
        }
    }

    fn tapped(&mut self, layer: &mut usize) {
        // initializer tapped state
        let keymap_index = (config::COLUMNS * self.row) + self.column;
        self.active_key = config::KEYMAP[*layer][usize::from(keymap_index)];
        self.timestamp_tapped = Instant::now();

        match self.active_key {
            Key::Layer(commanded_layer) => {
                // store base keycode
                resources::STATUS_SIGNAL.signal(resources::Status::Color(RGB8::new(0, 0, 50)));
                *layer = usize::from(commanded_layer);
            }
            Key::Pass => {
                let mut current_layer = *layer;
                while config::KEYMAP[current_layer][usize::from(keymap_index)] == Key::Pass
                    && current_layer >= 1
                {
                    current_layer -= 1;
                }
                self.active_key = config::KEYMAP[current_layer][usize::from(keymap_index)]
            }
            Key::Macro(callback) => {
                // call the macro
                callback();
            }
            Key::MT(_modifier, _keycode) => {
                // do nothing on a tap for a MT key
            }
            _ => (),
        };
    }

    fn held(&mut self) {
        if let Key::MT(modifier, _tap_key) = self.active_key {
            self.base_key = self.active_key;

            if (Instant::now() - self.timestamp_tapped).as_millis() >= config::MOD_TAP_THRESHOLD {
                self.active_key = Key::Mod(modifier)
            }
        }
    }

    fn released(&mut self, layer: &mut usize) {
        self.timestamp_released = Instant::now();

        // process
        match self.active_key {
            Key::Layer(_commanded_layer) => {
                // return our layer to the base layer
                *layer = 0;
                self.active_key = Key::Hid(Hid::Nop);
                resources::STATUS_SIGNAL.signal(resources::Status::Heartbeat);
            }
            Key::MT(_modifier, tap_key) => {
                if (self.timestamp_released - self.timestamp_tapped).as_millis()
                    < config::MOD_TAP_THRESHOLD
                {
                    self.active_key = Key::Hid(tap_key);
                } else {
                    self.active_key = self.base_key;
                }
            }
            _ => self.active_key = Key::Hid(Hid::Nop),
        }
    }

    fn idle(&mut self) {
        // during idle, check if our base key is MT, and reset back to the base
        if let Key::MT(_tap_key, _hold_key) = self.base_key {
            self.active_key = self.base_key;
            self.base_key = Key::Hid(Hid::Nop);
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

    pub fn get_keycode(&self) -> Option<Key> {
        match self.active_key {
            // filtering out custom keycodes that should never be sent to a computer
            Key::Hid(Hid::Nop) => None,
            Key::Layer(_commanded_layer) => None,
            Key::Pass => None,
            Key::Macro(_callback) => None,
            Key::MT(_mod, _keycode) => None,
            Key::Mod(_) => Some(self.active_key),
            Key::Hid(_) => Some(self.active_key),
        }
    }
}

// embassy
use embassy_time::Instant;

// crate
use crate::cc_engine::keycodes::{CCKeycode, CCModifier};
use crate::config;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    Pressed,
    Released,
}

pub struct KeyInfo {
    pub pressed: KeyState,
    pub keycode: Option<CCKeycode>,
    pub row: u8,
    pub column: u8,
    pub timestamp_pressed: Instant,
    pub timestamp_released: Instant,
}

impl KeyInfo {
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
}

pub trait KeyboardKey {
    fn new(row: u8, column: u8) -> Self;
    fn pressed(&mut self, layer: &mut usize);
    fn released(&mut self, layer: &mut usize);
    fn process(&mut self, layer: &mut usize);
    fn get_keycode(&self) -> Option<CCKeycode>;
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
            let keymap_index = (config::COLUMNS * self.row) + self.column;

            let keycode = config::KEYMAP[usize::from(*layer)][usize::from(keymap_index)];
            self.keycode = match keycode {
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
        } else if previous_pressed != self.pressed && self.pressed == KeyState::Released {
            // transitioning from pressed to released
            self.timestamp_released = Instant::now();

            // do keycode specific actions
            if let Some(keycode) = self.keycode {
                match keycode {
                    CCKeycode::LAYER(_commanded_layer) => {
                        // return our layer to the base layer
                        *layer = 0;
                    }
                    CCKeycode::MT(_modifier, keycode) => {
                        if (self.timestamp_pressed - self.timestamp_released).as_millis() < 200 {
                            // press the key
                            self.keycode = Some(CCKeycode::from(keycode));
                        }
                    }
                    _ => (),
                }
            }

            self.keycode = None
        } else {
            // handle keys that should be monitored continuously such as Mod Tap
            if let Some(keycode) = self.keycode {
                match keycode {
                    CCKeycode::MT(modifier, keycode) => {
                        // modtap support
                        if (Instant::now() - self.timestamp_pressed).as_millis() >= 200 {
                            // act as modifier
                            if (self.pressed == KeyState::Pressed) {
                                match modifier {
                                    CCModifier::L_CTRL => self.keycode = Some(CCKeycode::L__CTRL),
                                    CCModifier::L_SHFT => self.keycode = Some(CCKeycode::L_SHIFT),
                                    CCModifier::L_ALT => self.keycode = Some(CCKeycode::L___ALT),
                                    CCModifier::L_GUI => self.keycode = Some(CCKeycode::L___GUI),
                                    _ => self.keycode = None,
                                }
                            } else {
                                // time is greater than mod tap time and key is now released
                                self.keycode = None
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn get_keycode(&self) -> Option<CCKeycode> {
        if let Some(keycode) = self.keycode {
            match keycode {
                // filtering out custom keycodes that should never be sent to a computer
                CCKeycode::LAYER(_commanded_layer) => None,
                CCKeycode::PASSTHR => None,
                CCKeycode::MACRO(_callback) => None,
                CCKeycode::MT(_mod, _keycode) => None,
                _ => self.keycode,
            }
        } else {
            None
        }
    }
}

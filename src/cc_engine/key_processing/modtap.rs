use embassy_time::Instant;

use super::key::{self, KeyInfo, KeyState};
use crate::cc_engine::keycodes::{CCKeycode, CCModifier};
use crate::config;

pub struct ModTapKey {
    info: KeyInfo,
    modifier_active: bool,
}

impl key::KeyboardKey for ModTapKey {
    fn new(row: u8, column: u8) -> Self {
        Self { 
            info: KeyInfo::new(row, column),
            modifier_active: false
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
                CCKeycode::MT(modifier, keycode) => {
                    // will always be a mod tap key
                    Some(CCKeycode::MT(modifier, keycode))
                }
                _ => None, // should never happen
            };
        } else if previous_pressed != self.info.pressed && self.info.pressed == KeyState::Released {
            // transitioning from pressed to released
            self.info.timestamp_released = Instant::now();

            // do keycode specific actions
            if let Some(keycode) = self.info.keycode {
                match keycode {
                    CCKeycode::MT(_modifier, keycode) => {
                        if (self.info.timestamp_pressed - self.info.timestamp_released).as_millis()
                            < 200
                        {
                            // press the key
                            self.info.keycode = Some(CCKeycode::from(keycode));
                        }
                    }
                    _ => (),
                }
            }

            self.info.keycode = None
        } else {
            // handle keys that should be monitored continuously such as Mod Tap
            if let Some(keycode) = self.info.keycode {
                if let CCKeycode::MT(modifier, keycode) = 
                match keycode {
                    CCKeycode::MT(modifier, keycode) => {
                        // modtap support
                        if (Instant::now() - self.info.timestamp_pressed).as_millis() >= 200 {
                            // act as modifier
                            if (self.info.pressed == KeyState::Pressed) {
                                match modifier {
                                    CCModifier::L_CTRL => {
                                        self.info.keycode = Some(CCKeycode::L__CTRL)
                                    }
                                    CCModifier::L_SHFT => {
                                        self.info.keycode = Some(CCKeycode::L_SHIFT)
                                    }
                                    CCModifier::L_ALT => {
                                        self.info.keycode = Some(CCKeycode::L___ALT)
                                    }
                                    CCModifier::L_GUI => {
                                        self.info.keycode = Some(CCKeycode::L___GUI)
                                    }
                                    _ => self.info.keycode = None,
                                }
                            } else {
                                // time is greater than mod tap time and key is now released
                                self.info.keycode = None
                            }
                        }
                    }
                    _ => (),
                }
            }
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

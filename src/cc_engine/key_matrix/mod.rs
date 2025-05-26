use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pull};
use heapless::Vec;
use usbd_hid::descriptor::generator_prelude::Serialize;

use super::keycodes::{self, CCKeycode};
use crate::keymap::{self, KEYMAP};

#[allow(dead_code)]
pub enum DiodeDirection {
    ColumnToRow,
    RowToColumn,
}

#[derive(Copy, Clone, PartialEq)]
struct KeyIndex {
    pub keycode: CCKeycode,
    pub keymap_index: usize,
}
pub struct KeyProcessor {
    keycodes: [KeyIndex; keymap::ROWS * keymap::COLUMNS],
    index: usize,
    processing_layer: usize,
}

impl KeyProcessor {
    pub fn new() -> Self {
        Self {
            keycodes: [KeyIndex {
                keycode: CCKeycode::CC_NONE,
                keymap_index: 0,
            }; keymap::ROWS * keymap::COLUMNS],
            index: 0,
            processing_layer: 0,
        }
    }

    fn reset(&mut self) {
        self.index = 0;
    }

    fn push_key(&mut self, key: &CCKeycode, index: usize) {
        let key_index = KeyIndex {
            keycode: *key,
            keymap_index: index,
        };

        if !self.keycodes.contains(&key_index) {
            // add the keycode
            self.keycodes[self.index] = key_index;
            self.index += 1;
        }
    }

    pub fn pop_key(&mut self) -> CCKeycode {
        // grab the current index
        let mut key = self.keycodes[self.index];

        // convert the value on the current layer
        match key.keycode {
            // special case for a passthrough key
            CCKeycode::CC_PASS => key.keycode = KEYMAP[self.processing_layer - 1][key.keymap_index],
            _ => key.keycode = KEYMAP[self.processing_layer][key.keymap_index],
        }

        // decrement the index
        self.index -= 1;

        // return the key
        key.keycode
    }

    pub fn get_keycode_count(&self) -> usize {
        self.index
    }

    fn process(&mut self) {
        let mut current_layer: usize = 0;
        for index in 0..self.index {
            match self.keycodes[index].keycode {
                CCKeycode::CC_LAY(layer) => {
                    current_layer = layer;
                    self.keycodes[index].keycode = CCKeycode::CC_NONE;
                }
                _ => {
                    continue;
                }
            }
        }

        self.processing_layer = current_layer;
    }
}

pub struct KeyMatrix<const ROW_SIZE: usize, const COL_SIZE: usize> {
    rows: [AnyPin; ROW_SIZE],
    columns: [AnyPin; COL_SIZE],
    diode_direction: DiodeDirection,
    pressed_keys: [CCKeycode; keymap::ROWS * keymap::COLUMNS],
    active_layer: usize,
}

impl<const ROW_SIZE: usize, const COL_SIZE: usize> KeyMatrix<ROW_SIZE, COL_SIZE> {
    pub fn new(
        rows: [AnyPin; ROW_SIZE],
        columns: [AnyPin; COL_SIZE],
        diode_direction: DiodeDirection,
    ) -> Self {
        Self {
            columns,
            rows,
            diode_direction,
            pressed_keys: [CCKeycode::CC_NONE; keymap::ROWS * keymap::COLUMNS],
            active_layer: 0,
        }
    }

    // polls the matrix and returns up to 10 pressed keys
    pub fn poll(&mut self, processor: &mut KeyProcessor) {
        processor.reset();

        // poll the key matrix
        match self.diode_direction {
            DiodeDirection::ColumnToRow => {
                // poll by settign columns and readings rows
                for (col_index, col) in &mut self.columns.iter_mut().enumerate() {
                    // set the column high, should go back to low once this goes out of scope on the next iteration
                    let _output = Output::new(col, Level::High);

                    // poll the rows
                    for (row_index, row) in &mut self.rows.iter_mut().enumerate() {
                        let mut input = Input::new(row, Pull::None);
                        input.set_schmitt(true);
                        if input.is_high() {
                            // extrapolate the key from our keymap
                            let index = (keymap::COLUMNS * row_index) + col_index;
                            let key = &KEYMAP[self.active_layer][index];
                            processor.push_key(key, index);
                        }
                    }
                }
            }
            DiodeDirection::RowToColumn => {
                // poll by setting rows and reading columns
                for (row_index, row) in &mut self.rows.iter_mut().enumerate() {
                    // set the row high, should go back to low once this goes out of scope on the next iteration
                    let _output = Output::new(row, Level::High);

                    // poll the columns
                    for (col_index, col) in &mut self.columns.iter_mut().enumerate() {
                        let mut input = Input::new(col, Pull::None);
                        input.set_schmitt(true);
                        if input.is_high() {
                            // extrapolate the key from our keymap
                            let index = (keymap::COLUMNS * row_index) + col_index;
                            let key = &KEYMAP[self.active_layer][index];
                            processor.push_key(key, index);
                        }
                    }
                }
            }
        };

        // process the keys to do any conversions needed before other code will use them (such as layer shifts)
        processor.process();
    }
}

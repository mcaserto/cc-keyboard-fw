use embedded_hal::digital::{InputPin, OutputPin};
use rp2040_hal::gpio::PullDown;
use rp2040_hal::gpio::{DynPinId, FunctionSio, Pin, SioInput, SioOutput};

use super::keymap;

pub struct ActiveSwitch {
    pub column: usize,
    pub row: usize,
}

pub struct SwitchMatrix {
    // var col:
    matrix_columns: [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; keymap::COLUMNS],
    matrix_rows: [Pin<DynPinId, FunctionSio<SioInput>, PullDown>; keymap::ROWS],
    delay: cortex_m::delay::Delay,
}

impl SwitchMatrix {
    pub fn new(
        columns: [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; keymap::COLUMNS],
        rows: [Pin<DynPinId, FunctionSio<SioInput>, PullDown>; keymap::ROWS],
        delay: cortex_m::delay::Delay,
    ) -> Self {
        SwitchMatrix {
            matrix_columns: columns,
            matrix_rows: rows,
            delay,
        }
    }

    // return an array of max size 6 switches pressed
    pub fn poll(&mut self) -> [Option<ActiveSwitch>; 6] {
        // loop through column and read the rows
        let mut return_value: [Option<ActiveSwitch>; 6] = [None, None, None, None, None, None];
        let mut store_index: usize = 0;

        for (column_index, column) in self.matrix_columns.iter_mut().enumerate() {
            // set the column high
            column.set_high().unwrap();
            self.delay.delay_us(1);

            // read each column
            for (row_index, row) in self.matrix_rows.iter_mut().enumerate() {
                if row.is_high().unwrap() && store_index < 6 {
                    // key pressed
                    return_value[store_index] = Some(ActiveSwitch {
                        column: column_index,
                        row: row_index,
                    });
                    store_index += 1;
                }
            }

            // restore column to low
            column.set_low().unwrap();
        }
        return_value
    }
}

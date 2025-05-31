use embassy_rp::gpio::{Flex, Pull};

use crate::config;

use super::key::Key;

#[allow(dead_code)]
pub enum DiodeDirection {
    ColumnToRow,
    RowToColumn,
}

pub struct KeyboardMatrix<'a, const ROW_SIZE: usize, const COL_SIZE: usize> {
    rows: [Flex<'a>; ROW_SIZE],
    columns: [Flex<'a>; COL_SIZE],
    keys: [Key; config::COLUMNS as usize * config::ROWS as usize],
    diode_direction: DiodeDirection,
    active_layer: usize,
}

impl<'a, const ROW_SIZE: usize, const COL_SIZE: usize> KeyboardMatrix<'a, ROW_SIZE, COL_SIZE> {
    pub fn new(
        mut rows: [Flex<'a>; ROW_SIZE],
        mut columns: [Flex<'a>; COL_SIZE],
        diode_direction: DiodeDirection,
    ) -> Self {
        match &diode_direction {
            DiodeDirection::ColumnToRow => {
                // set rows as input and columns as output
                columns.iter_mut().for_each(|col| col.set_as_output());
                rows.iter_mut().for_each(|row| {
                    row.set_as_input();
                    row.set_pull(Pull::Down);
                });
            }
            DiodeDirection::RowToColumn => {
                // set rows as output and columns as input
                rows.iter_mut().for_each(|row| row.set_as_output());
                columns.iter_mut().for_each(|col| {
                    col.set_as_input();
                    col.set_pull(Pull::Down);
                });
            }
        }

        // generate keys
        let mut keys = [Key::new(0, 0); config::ROWS as usize * config::COLUMNS as usize];

        // instantiate key list
        keys.iter_mut().enumerate().for_each(|(index, key)| {
            let row = index / config::COLUMNS as usize;
            let column = index % config::COLUMNS as usize;
            *key = Key::new(row as u8, column as u8);
        });

        Self {
            columns,
            rows,
            keys,
            diode_direction,
            active_layer: 0,
        }
    }

    // polls the matrix and returns up to 10 pressed keys
    pub fn poll(&mut self) -> &[Key; config::COLUMNS as usize * config::ROWS as usize] {
        // poll the key matrix
        match self.diode_direction {
            DiodeDirection::ColumnToRow => {
                // poll by settign columns and readings rows
                for (col_index, col) in &mut self.columns.iter_mut().enumerate() {
                    col.set_high();
                    cortex_m::asm::delay(50);

                    // poll the rows
                    for (row_index, row) in &mut self.rows.iter_mut().enumerate() {
                        let adjusted_index = (usize::from(config::COLUMNS) * row_index) + col_index;
                        self.keys[adjusted_index].process(&row.is_high(), &mut self.active_layer);
                    }
                    col.set_low();
                }
            }
            DiodeDirection::RowToColumn => {
                // poll by setting rows and reading columns
                for (row_index, row) in &mut self.rows.iter_mut().enumerate() {
                    row.set_high();
                    cortex_m::asm::delay(50);

                    // poll the columns
                    for (col_index, col) in &mut self.columns.iter_mut().enumerate() {
                        let adjusted_index = (usize::from(config::COLUMNS) * row_index) + col_index;
                        self.keys[adjusted_index].process(&col.is_high(), &mut self.active_layer);
                    }
                    row.set_low();
                }
            }
        };

        &self.keys
    }
}

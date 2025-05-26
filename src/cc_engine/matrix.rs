use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pull};

#[allow(dead_code)]
pub enum DiodeDirection {
    ColumnToRow,
    RowToColumn,
}

pub struct KeyboardMatrix<const ROW_SIZE: usize, const COL_SIZE: usize> {
    rows: [AnyPin; ROW_SIZE],
    columns: [AnyPin; COL_SIZE],
    diode_direction: DiodeDirection,
}

impl<const ROW_SIZE: usize, const COL_SIZE: usize> KeyboardMatrix<ROW_SIZE, COL_SIZE> {
    pub fn new(
        rows: [AnyPin; ROW_SIZE],
        columns: [AnyPin; COL_SIZE],
        diode_direction: DiodeDirection,
    ) -> Self {
        Self {
            columns,
            rows,
            diode_direction,
        }
    }

    // polls the matrix and returns up to 10 pressed keys
    pub fn poll(&mut self) -> PollResult {
        let mut result = PollResult::new();

        // poll the key matrix
        match self.diode_direction {
            DiodeDirection::ColumnToRow => {
                // poll by settign columns and readings rows
                for (col_index, col) in &mut self.columns.iter_mut().enumerate() {
                    // set the column high, should go back to low once this goes out of scope on the next iteration
                    let _output = Output::new(col, Level::High);

                    // poll the rows
                    for (row_index, row) in &mut self.rows.iter_mut().enumerate() {
                        let mut input = Input::new(row, Pull::Down);
                        input.set_schmitt(true);
                        if input.is_high() {
                            // extrapolate the key from our keymap
                            // let index = (keymap::COLUMNS * row_index) + col_index;
                            result.push_key(row_index, col_index);
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
                        let mut input = Input::new(col, Pull::Down);
                        input.set_schmitt(true);
                        if input.is_high() {
                            // extrapolate the key from our keymap
                            // let index = (keymap::COLUMNS * row_index) + col_index;
                            result.push_key(row_index, col_index);
                        }
                    }
                }
            }
        };

        result
    }
}

#[derive(Clone, Copy)]
pub struct Key {
    pub row: usize,
    pub column: usize,
}

pub struct PollResult {
    key_stack: [Key; 10],
    stack_pointer: usize,
    exausted: bool,
}

impl PollResult {
    fn new() -> Self {
        Self {
            key_stack: [Key { row: 0, column: 0 }; 10],
            stack_pointer: 0,
            exausted: false,
        }
    }

    // push a new pressed key onto the stack
    fn push_key(&mut self, row: usize, col: usize) {
        if self.stack_pointer < self.key_stack.len() {
            self.key_stack[self.stack_pointer].row = row;
            self.key_stack[self.stack_pointer].column = col;
            self.stack_pointer += 1;
        }
    }

    pub fn pop_key(&mut self) -> Key {
        if self.stack_pointer > 0 {
            let key = self.key_stack[self.stack_pointer];
            self.stack_pointer -= 1;
            key
        } else {
            self.exausted = true;
            self.key_stack[self.stack_pointer]
        }
    }

    pub fn get_num_keys(&self) -> usize {
        if !self.exausted {
            self.stack_pointer
        } else {
            0
        }
    }
}

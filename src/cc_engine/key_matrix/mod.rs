use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pull};

#[allow(dead_code)]
pub enum DiodeDirection {
    ColumnToRow,
    RowToColumn,
}
pub struct KeyMatrix<const ROW_SIZE: usize, const COL_SIZE: usize> {
    rows: [AnyPin; ROW_SIZE],
    columns: [AnyPin; COL_SIZE],
    diode_direction: DiodeDirection,
}

#[derive(Clone, Copy)]
pub struct PressedKey {
    row: u16,
    column: u16,
}

pub struct PollResult {
    // results can be size 10 max
    index: usize,
    results: [PressedKey; 10],
}

impl PollResult {
    fn new() -> Self {
        PollResult {
            index: 0,
            results: [PressedKey { row: 0, column: 0 }; 10],
        }
    }

    fn push_result(&mut self, row: u16, col: u16) {
        if self.index == 10 {
            return;
        }
        self.results[self.index].row = row;
        self.results[self.index].column = col;
        self.index += 1;
    }

    pub fn pop_result(&mut self) -> PressedKey {
        if self.index == 0 {
            return self.results[0];
        }

        let res = self.results[self.index];
        self.index -= 1;
        res
    }

    pub fn get_result_count(&self) -> usize {
        self.index
    }
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
                        let mut input = Input::new(row, Pull::None);
                        input.set_schmitt(true);
                        if input.is_high() {
                            result.push_result(row_index as u16, col_index as u16);
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
                            result.push_result(row_index as u16, col_index as u16);
                        }
                    }
                }
            }
        };

        result
    }
}

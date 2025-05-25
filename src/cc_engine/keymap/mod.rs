use embassy_rp::gpio::AnyPin;

pub enum DiodeDirection {
    COLUMN_TO_ROW,
    ROW_TO_COLUMN,
}
pub struct KeyMatrix<const ROW_SIZE: usize, const COL_SIZE: usize> {
    rows: [AnyPin; ROW_SIZE],
    columns: [AnyPin; COL_SIZE],
    diode_direction: DiodeDirection,
}

pub struct PollResult<const SIZE: usize> {
    results: [bool; SIZE],
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

    // pub fn poll() -> PollResult {
    //     // poll the key matrix
    // }
}

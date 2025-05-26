use embassy_time::Timer;

use crate::cc_engine::{self, matrix};
// local includes
use crate::keymap;
use crate::tasks::resources;

// task for polling the keyboard matrix
#[embassy_executor::task]
pub async fn matrix_polling_handler(
    mut key_matrix: matrix::KeyboardMatrix<{ keymap::ROWS }, { keymap::COLUMNS }>,
) {
    // loop
    loop {
        let mut result = key_matrix.poll();

        // process
        let report = cc_engine::polling::process_poll_result(&mut result);
        resources::REPORT_CHANNEL.send(report).await;

        Timer::after_millis(1).await;
    }
}

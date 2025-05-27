use crate::cc_engine::{self, matrix};
use usbd_hid::descriptor;
// local includes
use super::resources;
use crate::keymap;

// task for polling the keyboard matrix
#[embassy_executor::task]
pub async fn matrix_polling_handler(
    mut key_matrix: matrix::KeyboardMatrix<{ keymap::ROWS }, { keymap::COLUMNS }>,
) {
    // loop
    let mut last_report = descriptor::KeyboardReport::default();
    loop {
        let mut result = key_matrix.poll().await;

        // process
        let report = cc_engine::processing::process_poll_result(&mut result);

        if report != last_report {
            resources::KEYBOARD_REPORT_CHANNEL.send(report).await;
            last_report = report;
        }
    }
}

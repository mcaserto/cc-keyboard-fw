use crate::cc_engine::{self, matrix};
use embassy_time::Timer;
use usbd_hid::descriptor::{self, KeyboardReport};

// crate includes
use super::resources;
use crate::config;

// task for polling the keyboard matrix
#[embassy_executor::task]
pub async fn matrix_polling_handler(
    mut key_matrix: matrix::KeyboardMatrix<
        'static,
        { config::ROWS as usize },
        { config::COLUMNS as usize },
    >,
) {
    // loop
    let mut last_report = KeyboardReport::default();
    loop {
        let result = key_matrix.poll();
        let report = cc_engine::processing::process_poll_result(result);

        // if last_report != report {
        resources::KEYBOARD_REPORT_CHANNEL.send(report).await;
        // }
        Timer::after_millis(1).await;
    }
}

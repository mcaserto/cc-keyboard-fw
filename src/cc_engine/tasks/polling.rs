use crate::cc_engine::{
    self, matrix,
    tasks::resources::{SUSPENDED, WAKE_SIGNAL},
};
use embassy_time::Timer;
use usbd_hid::descriptor::KeyboardReport;

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
    loop {
        let result = key_matrix.poll();
        let report = cc_engine::processing::process_keyboard_report(result);

        if !SUSPENDED.load(core::sync::atomic::Ordering::Acquire) {
            resources::REPORT_CHANNEL
                .send(resources::GenericHidReport::Keyboard(report))
                .await;
        } else {
            // if not awake, send a wake signal if any key is pressed
            if report != KeyboardReport::default() {
                WAKE_SIGNAL.signal(());
                SUSPENDED.store(false, core::sync::atomic::Ordering::Release);
            }
        }

        Timer::after_millis(1).await;
    }
}

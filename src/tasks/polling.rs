use embassy_time::Timer;
use usbd_hid::descriptor::KeyboardReport;

// local includes
use crate::key_matrix;
use crate::keymap;
use crate::tasks::resources;

// task for polling the keyboard matrix
#[embassy_executor::task]
pub async fn matrix_polling_handler(
    mut keymap: key_matrix::KeyMatrix<{ keymap::ROWS }, { keymap::COLUMNS }>,
) {
    // do setup

    let mut last_report_zero: bool = false;

    // loop
    loop {
        // Set up the signal pin that will be used to trigger the keyboard.
        // let mut signal_pin = Input::new(p.PIN_16, Pull::None);

        // Enable the schmitt trigger to slightly debounce.
        // signal_pin.set_schmitt(true);

        // control.send(LedState::Toggle).await;
        // ticker.next().await;
        let result = keymap.poll();

        if result.get_result_count() > 0 {
            // send it
            // just write an HID report for now
            let report = KeyboardReport {
                keycodes: [4, 0, 0, 0, 0, 0],
                leds: 0,
                modifier: 0,
                reserved: 0,
            };
            resources::REPORT_CHANNEL.send(report).await;
            last_report_zero = false;
        } else {
            // send an empty report
            if !last_report_zero {
                let report = KeyboardReport {
                    keycodes: [0, 0, 0, 0, 0, 0],
                    leds: 0,
                    modifier: 0,
                    reserved: 0,
                };
                resources::REPORT_CHANNEL.send(report).await;
                last_report_zero = true;
            }
        }

        Timer::after_millis(1).await;
    }
}

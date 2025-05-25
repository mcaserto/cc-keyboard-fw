use embassy_time::Timer;
use usbd_hid::descriptor::KeyboardReport;

use crate::tasks::resources;

// task for polling the keyboard matrix
#[embassy_executor::task]
pub async fn matrix_polling_handler() {
    // do setup

    // loop
    loop {
        // Set up the signal pin that will be used to trigger the keyboard.
        // let mut signal_pin = Input::new(p.PIN_16, Pull::None);

        // Enable the schmitt trigger to slightly debounce.
        // signal_pin.set_schmitt(true);

        // control.send(LedState::Toggle).await;
        // ticker.next().await;

        // just write an HID report for now
        let report = KeyboardReport {
            keycodes: [4, 0, 0, 0, 0, 0],
            leds: 0,
            modifier: 0,
            reserved: 0,
        };

        resources::REPORT_CHANNEL.send(report).await;

        Timer::after_secs(1).await;
    }
}

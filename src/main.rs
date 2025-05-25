#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use panic_probe as _;
use {defmt_rtt as _, panic_probe as _};

mod tasks;
use tasks::polling;
use tasks::status;
use tasks::usb;

mod cc_engine;
use cc_engine::key_matrix;

mod keymap;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // create a key matrix
    let rows = [
        p.PIN_0.degrade(),
        p.PIN_1.degrade(),
        p.PIN_2.degrade(),
        p.PIN_3.degrade(),
    ];
    let cols = [
        p.PIN_4.degrade(),
        p.PIN_5.degrade(),
        p.PIN_6.degrade(),
        p.PIN_7.degrade(),
        p.PIN_8.degrade(),
        p.PIN_9.degrade(),
        p.PIN_29.degrade(),
        p.PIN_28.degrade(),
        p.PIN_27.degrade(),
        p.PIN_26.degrade(),
        p.PIN_18.degrade(),
        p.PIN_20.degrade(),
    ];

    // create keymap from rows and columns
    let mut key_matrix =
        key_matrix::KeyMatrix::new(rows, cols, key_matrix::DiodeDirection::ColumnToRow);

    // initialize usb tasks
    usb::initialize_usb_resources(p.USB, &spawner);

    // start status led task
    spawner.must_spawn(status::status_light_handler(p.PIN_17, p.PIO0, p.DMA_CH0));

    // start keyboard matrix polling task
    spawner.must_spawn(polling::matrix_polling_handler(key_matrix));
}

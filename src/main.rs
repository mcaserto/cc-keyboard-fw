#![no_std]
#![no_main]

use embassy_rp::gpio::Pin;
use panic_probe as _;
use {defmt_rtt as _, panic_probe as _};

use cc_engine::tasks::polling;
use cc_engine::tasks::status;
use cc_engine::tasks::usb;

mod cc_engine;
use cc_engine::matrix;

mod keymap;

#[cortex_m_rt::entry]
fn main() -> ! {
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
    let key_matrix = matrix::KeyboardMatrix::new(rows, cols, matrix::DiodeDirection::ColumnToRow);

    // spawn tasks onto the desired cores
    use embassy_executor::Executor;
    use embassy_rp::multicore;
    use static_cell::StaticCell;
    static mut CORE1_STACK: multicore::Stack<4096> = multicore::Stack::new();
    static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
    static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

    // spawn the core 1 tasks (just matrix polling for now)
    embassy_rp::multicore::spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1
                .run(|spawner| spawner.must_spawn(polling::matrix_polling_handler(key_matrix)));
        },
    );

    // start the core 0 tasks (just status led and usb handling for now)
    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        spawner.must_spawn(status::status_light_handler(p.PIN_17, p.PIO0, p.DMA_CH0));
        usb::initialize_usb_resources(p.USB, &spawner);
    });
}

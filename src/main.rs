#![no_std]
#![no_main]

// embassy-rs includes
use embassy_executor::Executor;
use embassy_rp::gpio::Flex;
use embassy_rp::multicore;
use embassy_rp::clocks::ClockConfig;
use embassy_rp::config::Config;

// panic handler, logging, etc.
use panic_probe as _;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// include code from the cc_engine
mod cc_engine;
use cc_engine::matrix;
use cc_engine::tasks::polling;
use cc_engine::tasks::status;
use cc_engine::tasks::usb;

// our keyboard definitions
mod config;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Set up for clock frequency of 200 MHz, setting all necessary defaults.
    let config = Config::new(ClockConfig::system_freq(200_000_000).unwrap());
    let p = embassy_rp::init(Config::default());

    // create a key matrix ( ideally I want this to be set up in the keymap.rs file so everything is configured there )
    let rows = [
        Flex::new(p.PIN_0),
        Flex::new(p.PIN_1),
        Flex::new(p.PIN_2),
        Flex::new(p.PIN_3),
    ];
    let cols = [
        Flex::new(p.PIN_4),
        Flex::new(p.PIN_5),
        Flex::new(p.PIN_6),
        Flex::new(p.PIN_7),
        Flex::new(p.PIN_8),
        Flex::new(p.PIN_9),
        Flex::new(p.PIN_29),
        Flex::new(p.PIN_28),
        Flex::new(p.PIN_27),
        Flex::new(p.PIN_26),
        Flex::new(p.PIN_18),
        Flex::new(p.PIN_20),
    ];

    // create keymap from rows and columns
    let key_matrix = matrix::KeyboardMatrix::new(rows, cols, matrix::DiodeDirection::ColumnToRow);

    // spawn tasks onto the desired cores
    static mut CORE1_STACK: multicore::Stack<10258> = multicore::Stack::new();
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

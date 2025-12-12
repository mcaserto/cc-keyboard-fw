// embassy includes
use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_rp::peripherals::WATCHDOG;
use embassy_rp::watchdog::Watchdog;
use embassy_rp::{peripherals, usb, Peri};
use embassy_time::Instant;
use embassy_usb::class::hid;
use embassy_usb::control;
use static_cell::StaticCell;
use usbd_hid::descriptor::{AsInputReport, KeyboardReport, SerializedDescriptor};

use crate::cc_engine::tasks::resources::{GenericHidReport, STATUS_SIGNAL, SUSPENDED, WAKE_SIGNAL};

// local includes
use super::resources;

embassy_rp::bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<peripherals::USB>;
});

// task for setting up the usb handler and processing usb events
pub fn initialize_usb_resources(
    usb_peripheral: Peri<'static, peripherals::USB>,
    watchdog: Peri<'static, WATCHDOG>,
    spawner: &Spawner,
) {
    // Create the driver, from the HAL.
    let driver = usb::Driver::new(usb_peripheral, Irqs);

    // Create embassy-usb Config
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("CC LLE");
    config.product = Some("CC Keyboard");
    config.serial_number = Some("00000001");
    config.max_power = 100;
    config.max_packet_size_0 = 64;
    config.supports_remote_wakeup = true;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    static MSOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        CONFIG_DESCRIPTOR.init([0; 256]),
        BOS_DESCRIPTOR.init([0; 256]),
        MSOS_DESCRIPTOR.init([0; 256]),
        CONTROL_BUF.init([0; 64]),
    );

    static DEVICE_HANDLER: StaticCell<MyDeviceHandler> = StaticCell::new();
    builder.handler(DEVICE_HANDLER.init(MyDeviceHandler::new()));

    // Create classes on the builder.
    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 64,
        hid_subclass: hid::HidSubclass::No,
        hid_boot_protocol: hid::HidBootProtocol::Keyboard,
    };

    static STATE: StaticCell<hid::State> = StaticCell::new();
    let hid =
        hid::HidReaderWriter::<_, 1, 8>::new(&mut builder, STATE.init(hid::State::new()), config);

    // Build the builder.
    let usb = builder.build();

    let (reader, writer) = hid.split();

    // start the writer task
    spawner.spawn(usb_out_handler(writer).unwrap());

    // start the reader task
    spawner.spawn(usb_in_handler(reader).unwrap());

    // start the usb handler
    spawner.spawn(usb_handler(usb, watchdog).unwrap());
}

#[embassy_executor::task]
async fn usb_out_handler(
    mut writer: hid::HidWriter<'static, usb::Driver<'static, peripherals::USB>, 8>,
) {
    loop {
        // wait for a new keyboard report
        info!("Waiting keyboard report");
        let report = resources::REPORT_CHANNEL.receive().await;

        match &report {
            GenericHidReport::Keyboard(keyboard_report) => {
                write_report(keyboard_report, &mut writer).await;
            }
            GenericHidReport::Mouse(mouse_report) => {
                write_report(mouse_report, &mut writer).await;
            }
            GenericHidReport::Media(media_report) => {
                write_report(media_report, &mut writer).await;
            }
            GenericHidReport::System(system_control_report) => {
                write_report(system_control_report, &mut writer).await;
            }
        }
        info!("Sent report");
    }
}

async fn write_report<T: AsInputReport>(
    report: &T,
    writer: &mut hid::HidWriter<'static, usb::Driver<'static, peripherals::USB>, 8>,
) {
    if !SUSPENDED.load(Ordering::Acquire) {
        match writer.write_serialize(report).await {
            Ok(()) => {}
            Err(e) => warn!("Failed to send report: {:?}", e),
        };
    }
}

#[embassy_executor::task]
async fn usb_in_handler(
    reader: hid::HidReader<'static, usb::Driver<'static, peripherals::USB>, 1>,
) {
    let mut request_handler = MyRequestHandler {};

    loop {
        reader.run(false, &mut request_handler).await;
    }
}

#[embassy_executor::task]
async fn usb_handler(
    mut usb: embassy_usb::UsbDevice<'static, usb::Driver<'static, peripherals::USB>>,
    watchdog: Peri<'static, WATCHDOG>,
) {
    let mut watchdog_timer = Watchdog::new(watchdog);

    // run the usb device
    loop {
        usb.run_until_suspend().await;

        // // now suspended, wait for wakeup
        match select(usb.wait_resume(), WAKE_SIGNAL.wait()).await {
            Either::First(_) => (),
            Either::Second(_) => {
                // set remote wakeup bit in usb registers ( embassy HAL doesn't support this for some reason? )
                embassy_rp::pac::USB
                    .sie_ctrl()
                    .write(|w| w.set_resume(true));
                // embassy_rp::pac::USB.inte().write(|r| r.set_dev_sof(true));

                let start = Instant::now();
                while Instant::now().duration_since(start).as_secs() <= 1 {}
                watchdog_timer.trigger_reset(); // hacky as hell but easier than fixing broken usb state for now
            }
        }
    }
}

struct MyRequestHandler {}

impl hid::RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: hid::ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: hid::ReportId, data: &[u8]) -> control::OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        control::OutResponse::Accepted
    }

    fn get_protocol(&self) -> hid::HidProtocolMode {
        // TODO! Implement this
        hid::HidProtocolMode::Report
    }

    fn set_protocol(&mut self, _protocol: hid::HidProtocolMode) -> control::OutResponse {
        // TODO! Implement this
        control::OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<hid::ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<hid::ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl embassy_usb::Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        SUSPENDED.store(false, Ordering::Release);
        if enabled {
            info!("Device enabled");
        } else {
            STATUS_SIGNAL.signal(resources::Status::Error);
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
            STATUS_SIGNAL.signal(resources::Status::Error);
        }
    }

    fn suspended(&mut self, suspended: bool) {
        if suspended {
            SUSPENDED.store(true, Ordering::Release);
        } else {
            SUSPENDED.store(false, Ordering::Release);
        }
    }
}

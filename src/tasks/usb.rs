// embassy includes
use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{peripherals, usb};
use embassy_usb::class::hid;
use embassy_usb::control;
use static_cell::StaticCell;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

// local includes
use crate::tasks::resources;

embassy_rp::bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<peripherals::USB>;
});

// task for setting up the usb handler and processing usb events
pub fn initialize_usb_resources(usb_peripheral: peripherals::USB, spawner: &Spawner) {
    // Create the driver, from the HAL.
    let driver = usb::Driver::new(usb_peripheral, Irqs);

    // Create embassy-usb Config
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Caserto LLE");
    config.product = Some("CC Keyboard");
    config.serial_number = Some("00000001");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

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
        poll_ms: 60,
        max_packet_size: 64,
    };

    static STATE: StaticCell<hid::State> = StaticCell::new();
    let hid =
        hid::HidReaderWriter::<_, 1, 8>::new(&mut builder, STATE.init(hid::State::new()), config);

    // Build the builder.
    let usb = builder.build();

    let (reader, writer) = hid.split();

    // start the writer task
    spawner.must_spawn(usb_out_handler(writer));

    // start the reader task
    spawner.must_spawn(usb_in_handler(reader));

    // start the usb handler
    spawner.must_spawn(usb_handler(usb));
}

#[embassy_executor::task]
async fn usb_out_handler(
    mut writer: hid::HidWriter<'static, usb::Driver<'static, peripherals::USB>, 8>,
) {
    loop {
        // wait for a new keyboard report
        info!("Waiting keyboard report");
        let report = resources::REPORT_CHANNEL.receive().await;

        match writer.write_serialize(&report).await {
            Ok(()) => {}
            Err(e) => warn!("Failed to send report: {:?}", e),
        };
        info!("Sent report");
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
) {
    // run the usb device
    usb.run().await;
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
        if enabled {
            info!("Device enabled");
        } else {
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
        }
    }
}

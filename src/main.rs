//! # Keyboard firmware for custom RP2040 based keyboard

#![no_std]
#![no_main]

use adafruit_kb2040::entry;

// The macro for marking our interrupt functions
use adafruit_kb2040::hal::pac::interrupt;

use core::iter::once;
use panic_halt as _;

use adafruit_kb2040::hal::{self};
use adafruit_kb2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    XOSC_CRYSTAL_FREQ,
};
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Human Interface Device (HID) Class support
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;

// custom keyboard code
mod cckeyboard;
use cckeyboard::matrix::SwitchMatrix;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

/// Entry point to our bare-metal application.
#[entry]
fn main() -> ! {
    // Configure the RP2040 peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // set up timer
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    // Grab a reference to the USB Bus allocator
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Set up the USB HID Class Device driver, providing Mouse Reports
    let usb_hid = HIDClass::new(bus_ref, KeyboardReport::desc(), 60);
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet.
        USB_HID = Some(usb_hid);
    }

    // Create a USB device with a fake VID and PID
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27da))
        .strings(&[StringDescriptors::default()
            .manufacturer("CC Peripherals")
            .product("CC-Keyboard")
            .serial_number("TEST")])
        .unwrap()
        .device_class(0)
        .build();
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    unsafe {
        // Enable the USB interrupt
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };

    let sio = Sio::new(pac.SIO);

    let pins = adafruit_kb2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure the addressable LED
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let mut ws = Ws2812::new(
        pins.neopixel.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let core = pac::CorePeripherals::take().unwrap();
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    // let mut timer = timer; // rebind to force a copy of the timer

    // get columns
    let c0 = pins.d4.into_push_pull_output().into_dyn_pin();
    let c1 = pins.d5.into_push_pull_output().into_dyn_pin();
    let c2 = pins.d6.into_push_pull_output().into_dyn_pin();
    let c3 = pins.d7.into_push_pull_output().into_dyn_pin();
    let c4 = pins.d8.into_push_pull_output().into_dyn_pin();
    let c5 = pins.d9.into_push_pull_output().into_dyn_pin();
    let c6 = pins.a3.into_push_pull_output().into_dyn_pin();
    let c7 = pins.a2.into_push_pull_output().into_dyn_pin();
    let c8 = pins.a1.into_push_pull_output().into_dyn_pin();
    let c9 = pins.a0.into_push_pull_output().into_dyn_pin();
    let c10 = pins.sclk.into_push_pull_output().into_dyn_pin();
    let c11 = pins.miso.into_push_pull_output().into_dyn_pin();

    // get rows
    let r0 = pins.tx.into_pull_down_input().into_dyn_pin();
    let r1 = pins.rx.into_pull_down_input().into_dyn_pin();
    let r2 = pins.d2.into_pull_down_input().into_dyn_pin();
    let r3 = pins.d3.into_pull_down_input().into_dyn_pin();

    let columns = [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11];
    let rows = [r0, r1, r2, r3];

    let mut matrix = SwitchMatrix::new(columns, rows, delay);

    loop {
        ws.write(once(RGB8::new(10, 10, 0))).unwrap();

        let keys_pressed = matrix.poll();

        // extract a report
        let report = cckeyboard::process_keys(keys_pressed);

        input_key_press(report).unwrap_or(0);
    }
}

fn input_key_press(report: KeyboardReport) -> Result<usize, usb_device::UsbError> {
    critical_section::with(|_| unsafe {
        // Now interrupts are disabled, grab the global variable and, if
        // available, send it a HID report
        USB_HID.as_mut().map(|hid| hid.push_input(&report))
    })
    .unwrap()
}

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Handle USB request
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_dev.poll(&mut [usb_hid]);
}

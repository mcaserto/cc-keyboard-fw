// resources.rs
// description: contains shared resources for use between tasks

use core::sync::atomic::AtomicBool;

// embassy includes
use embassy_sync::channel::Channel;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use smart_leds::RGB8;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport, SystemControlReport};

#[allow(dead_code)]
pub enum GenericHidReport {
    Keyboard(KeyboardReport),
    Media(MediaKeyboardReport),
    Mouse(MouseReport),
    System(SystemControlReport),
}

// ridiculous buffer sizes but I've got memory so why not
pub static REPORT_CHANNEL: Channel<CriticalSectionRawMutex, GenericHidReport, 100> = Channel::new();

pub static STATUS_SIGNAL: Signal<CriticalSectionRawMutex, Status> = Signal::new();

pub static WAKE_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub static SUSPENDED: AtomicBool = AtomicBool::new(false);

#[allow(dead_code)]
pub enum Status {
    Idle,
    Error,
    Heartbeat,
    Color(RGB8),
}

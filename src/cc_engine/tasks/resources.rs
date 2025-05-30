// resources.rs
// description: contains shared resources for use between tasks

// embassy includes
use embassy_sync::channel::Channel;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport, SystemControlReport};

// ridiculous buffer sizes but I've got memory so why not
pub static KEYBOARD_REPORT_CHANNEL: Channel<CriticalSectionRawMutex, KeyboardReport, 100> =
    Channel::new();
pub static MEDIA_REPORT_CHANNEL: Channel<CriticalSectionRawMutex, MediaKeyboardReport, 100> =
    Channel::new();
pub static MOUSE_REPORT_CHANNEL: Channel<CriticalSectionRawMutex, MouseReport, 100> =
    Channel::new();
pub static SYSTEM_CONTROL_REPORT_CHANNEL: Channel<
    CriticalSectionRawMutex,
    SystemControlReport,
    100,
> = Channel::new();

pub static STATUS_SIGNAL: Signal<CriticalSectionRawMutex, StatusCode> = Signal::new();

pub enum StatusCode {
    Error,
    Warning,
}

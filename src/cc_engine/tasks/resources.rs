// resources.rs
// description: contains shared resources for use between tasks

// embassy includes
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport, SystemControlReport};

// ridiculous buffer sizes but I've got memory so why not
pub static KEYBOARD_REPORT_CHANNEL: Channel<ThreadModeRawMutex, KeyboardReport, 100> =
    Channel::new();
pub static MEDIA_REPORT_CHANNEL: Channel<ThreadModeRawMutex, MediaKeyboardReport, 100> =
    Channel::new();
pub static MOUSE_REPORT_CHANNEL: Channel<ThreadModeRawMutex, MouseReport, 100> = Channel::new();
pub static SYSTEM_CONTROL_REPORT_CHANNEL: Channel<ThreadModeRawMutex, SystemControlReport, 100> =
    Channel::new();

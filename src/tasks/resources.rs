// resources.rs
// description: contains shared resources for use between tasks

// embassy includes
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use usbd_hid::descriptor::KeyboardReport;

pub static REPORT_CHANNEL: Channel<ThreadModeRawMutex, KeyboardReport, 10> = Channel::new();

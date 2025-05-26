# CC-Keyboard Firmware

RP2040 based keyboard firmware written in rust. The CC-Keyboard is my custom designed otherlinear 12 column, 4 row keyboard.

## Current Features
- Basic HID keyboard functionality
- Customizable keymaps
- Dual Core, primary core used for usb and status light, second core is used for polling the keyboard matrix.

## Future plans
- Macros
- All configuration done in root keymap.rs file. Currently this is only configuring the keymap. I'd like pin definitions to be here as well.
- Custom keycodes for sending media key events, system events (pc sleep), and even mouse input would be nice. Custom keycodes for macros, bootloader, etc.

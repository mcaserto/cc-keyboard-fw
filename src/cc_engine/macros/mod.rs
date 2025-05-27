#[allow(dead_code)]
pub mod macro_helpers {
    use crate::cc_engine::{keycodes, tasks::resources};
    use usbd_hid::descriptor::KeyboardReport;

    pub fn send_string(str: &str) {
        let mut report = KeyboardReport::default();
        for character in str.chars() {
            if character.is_uppercase() {
                report.modifier |= keycodes::CCModifier::CC_SHFT as u8;
                let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);
            } else {
                report.modifier = 0;
                let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);
            }

            // ignoring send error for now
            report.keycodes[0] = (keycodes::CCKeycode::from(character)).into();
            let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);

            // hacky to allow sending multiples of a single chracter
            report.keycodes[0] = 0;
            let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);
        }

        report = KeyboardReport::default();
        let _ = resources::KEYBOARD_REPORT_CHANNEL.try_send(report);
    }
}

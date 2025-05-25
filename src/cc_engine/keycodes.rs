// File: keycodes.rs
// Description: Contains all valid keycodes for cc_engine keyboard firmware
use usbd_hid::descriptor::KeyboardUsage::*;

#[repr(u8)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum CCKeycode {
    CC___A = KeyboardAa as u8,
    CC___B = KeyboardBb as u8,
    CC___C = KeyboardCc as u8,
    CC___D = KeyboardDd as u8,
    CC___E = KeyboardEe as u8,
    CC___F = KeyboardFf as u8,
    CC___G = KeyboardGg as u8,
    CC___H = KeyboardHh as u8,
    CC___I = KeyboardIi as u8,
    CC___J = KeyboardJj as u8,
    CC___K = KeyboardKk as u8,
    CC___L = KeyboardLl as u8,
    CC___M = KeyboardMm as u8,
    CC___N = KeyboardNn as u8,
    CC___O = KeyboardOo as u8,
    CC___P = KeyboardPp as u8,
    CC___Q = KeyboardQq as u8,
    CC___R = KeyboardRr as u8,
    CC___S = KeyboardSs as u8,
    CC___T = KeyboardTt as u8,
    CC___U = KeyboardUu as u8,
    CC___V = KeyboardVv as u8,
    CC___W = KeyboardWw as u8,
    CC___X = KeyboardXx as u8,
    CC___Y = KeyboardYy as u8,
    CC___Z = KeyboardZz as u8,
    CC___0 = Keyboard0CloseParens as u8,
    CC___1 = Keyboard1Exclamation as u8,
    CC___2 = Keyboard2At as u8,
    CC___3 = Keyboard3Hash as u8,
    CC___4 = Keyboard4Dollar as u8,
    CC___5 = Keyboard5Percent as u8,
    CC___6 = Keyboard6Caret as u8,
    CC___7 = Keyboard7Ampersand as u8,
    CC___8 = Keyboard8Asterisk as u8,
    CC___9 = Keyboard9OpenParens as u8,
    CC_RET = KeyboardReturn as u8,
    CC_ESC = KeyboardEscape as u8,
    CC_BSP = KeyboardBackspace as u8,
    CC_TAB = KeyboardTab as u8,
    CC_BAR = KeyboardSpacebar as u8,

    // custom codes for CC Firmware
    CC_NON,
    CC_LAY(u8), // layer shift key
}

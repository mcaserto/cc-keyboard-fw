// File: keycodes.rs
// Description: Contains all valid keycodes for cc_engine keyboard firmware
use usbd_hid::descriptor::KeyboardUsage::*;

#[repr(u8)]
#[allow(dead_code)]
pub enum CC_KEYCODE {
    CC_A = KeyboardAa as u8,
    CC_B = KeyboardBb as u8,
    CC_C = KeyboardCc as u8,
    CC_D = KeyboardDd as u8,
    CC_E = KeyboardEe as u8,
    CC_F = KeyboardFf as u8,
    CC_G = KeyboardGg as u8,
    CC_H = KeyboardHh as u8,
    CC_I = KeyboardIi as u8,
    CC_J = KeyboardJj as u8,
    CC_K = KeyboardKk as u8,
    CC_L = KeyboardLl as u8,
    CC_M = KeyboardMm as u8,
    CC_N = KeyboardNn as u8,
    CC_O = KeyboardOo as u8,
    CC_P = KeyboardPp as u8,
    CC_Q = KeyboardQq as u8,
    CC_R = KeyboardRr as u8,
    CC_S = KeyboardSs as u8,
    CC_T = KeyboardTt as u8,
    CC_U = KeyboardUu as u8,
    CC_V = KeyboardVv as u8,
    CC_W = KeyboardWw as u8,
    CC_X = KeyboardXx as u8,
    CC_Y = KeyboardYy as u8,
    CC_Z = KeyboardZz as u8,
}

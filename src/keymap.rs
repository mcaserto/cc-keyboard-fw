// File: keymap.rs
// Description: Contains information about the keyboard keymap
use crate::cc_engine::keycodes::{CCKeycode, CCKeycode::*};

pub const ROWS: usize = 4;
pub const COLUMNS: usize = 12;
pub const NUM_LAYERS: usize = 3;

#[allow(non_camel_case_types)]
type CCLayer = [CCKeycode; ROWS * COLUMNS];
type CCKeymap = [CCLayer; NUM_LAYERS];

#[rustfmt::skip]
pub const KEYMAP: CCKeymap = [
[
    CC__TAB, CC____Q, CC____W, CC____E, CC____R,    CC____T, CC____Y, CC____U,    CC____I, CC____O, CC____P, CC_BKSP,
    CC_BKSP, CC____A, CC____S, CC____D, CC____F,    CC____G, CC____H, CC____J,    CC____K, CC____L, CC_SCLN, CC_QUOT,
    CC_SHFT, CC____Z, CC____X, CC____C, CC____V,    CC____B, CC____N, CC____M,    CC_COMA, CC__PER, CC_SLSH, CC__RET,
    CC__ESC, CC_CTRL, CC__ALT, CC__GUI, CC_LAY(1),  CC_SPAC, CC_SPAC, CC_LAY(2),  CC_LFTA, CC_DWNA, CC_RGTA, CC__UPA,
],
[
    CC____1, CC____2, CC____3, CC____4, CC____5, CC____6, CC____7, CC____8,  CC____9, CC____O, CC_PASS, CC_PASS,
    CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS,  CC_PASS, CC_PASS, CC_PASS, CC_PASS,
    CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS,  CC_PASS, CC_PASS, CC_PASS, CC_PASS,
    CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS,  CC_PASS, CC_PASS, CC_PASS, CC_PASS,
],
[
    CC____1, CC____2, CC____3, CC____4, CC____5, CC____6, CC____7, CC____8,  CC____9, CC____O, CC_PASS, CC_PASS,
    CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS,  CC_PASS, CC_PASS, CC_PASS, CC_PASS,
    CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS,  CC_PASS, CC_PASS, CC_PASS, CC_PASS,
    CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS, CC_PASS,  CC_PASS, CC_PASS, CC_PASS, CC_PASS,
]
];

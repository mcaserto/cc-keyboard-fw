// File: keymap.rs
// Description: Contains information about the keyboard keymap
use crate::cc_engine::keycodes::CC_KEYCODE::{self, *};

pub const ROWS: usize = 4;
pub const COLUMNS: usize = 12;
pub const NUM_LAYERS: usize = 2;

type CC_LAYER = [CC_KEYCODE; ROWS * COLUMNS];
type CC_KEYMAP = [CC_LAYER; { NUM_LAYERS }];

#[rustfmt::skip]
pub const KEYMAP: CC_KEYMAP = [
[
    CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A,
    CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A,
    CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A,
    CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A,
],
[
    CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A,
    CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A,
    CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A,
    CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A, CC_A,
]
];

// File: keymap.rs
// Description: Contains information about the keyboard keymap
use crate::cc_engine::keycodes::{CCKeycode, CCKeycode::*};

pub const ROWS: usize = 4;
pub const COLUMNS: usize = 12;
pub const NUM_LAYERS: usize = 2;

#[allow(non_camel_case_types)]
type CCLayer = [CCKeycode; ROWS * COLUMNS];
type CCKeymap = [CCLayer; NUM_LAYERS];

#[rustfmt::skip]
pub const KEYMAP: CCKeymap = [
[
    CC_TAB, CC___Q, CC___W, CC___E, CC___R, CC___T, CC___Y, CC___U, CC___I, CC___O, CC___P, CC_BSP,
    CC_BSP, CC___A, CC___S, CC___D, CC___F, CC___G, CC___H, CC___J, CC___K, CC___L, CC_NON, CC_NON,
    CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A,
    CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A,
],
[
    CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A,
    CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A,
    CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A,
    CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A, CC___A,
]
];

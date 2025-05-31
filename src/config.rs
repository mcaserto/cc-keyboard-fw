// File: keymap.rs
// Description: Contains information about the keyboard keymap
use crate::cc_engine::keycodes::CCKeycode::{self, *};

pub const ROWS: u8 = 4;
pub const COLUMNS: u8 = 12;
pub const NUM_LAYERS: u8 = 3;

// other consts
pub const MOD_TAP_THRESHOLD: u64 = 250; // value in ms

#[allow(non_camel_case_types)]
type CCLayer = [CCKeycode; ROWS as usize * COLUMNS as usize];
type CCKeymap = [CCLayer; NUM_LAYERS as usize];

#[rustfmt::skip]
pub const KEYMAP: CCKeymap = [
[
    __TAB__, ___Q___, ___W___, ___E___, ___R___,         ___T___, ___Y___, ___U___,         ___I___, ___O___, ___P___, BACK_SP,
    BACK_SP, ___A___, ___S___, ___D___, MT(0x09, 0xE1),  ___G___, ___H___, MT(0x09, 0xE1),  ___K___, ___L___, Semicln, _QUOTE_,
    Lshft,   ___Z___, ___X___, ___C___, ___V___,         ___B___, ___N___, ___M___,         _COMMA_, __DOT__, _SLASH_, _ENTER_,
    __ESC__, Lctrl,   Lalt,    Lgui,    Layer(1),        _SPACE_, _SPACE_, Layer(2),        _LEFT__, _DOWN__, _RIGHT_, __UP___,
],
[
    Passthr,   Passthr, Passthr, Passthr, Passthr, Passthr, Passthr, Passthr,  Passthr, Passthr, Passthr, Passthr,
    Passthr,   FUNCT01, FUNCT02, FUNCT03, FUNCT04, FUNCT05, _LEFT__, _DOWN__,  __UP___, _RIGHT_, Passthr, Passthr,
    Passthr,   FUNCT07, FUNCT08, FUNCT09, FUNCT10, FUNCT11, FUNCT12, Passthr,  Passthr, Passthr, Passthr, Passthr,
    Macro(m1), Passthr, Passthr, Passthr, Passthr, Passthr, Passthr, Passthr,  Passthr, Passthr, Passthr, Passthr,
],
[
    Passthr,   ___0___, ___1___, ___2___, ___3___, Passthr, Passthr, Passthr,  Passthr, Passthr, Passthr, Passthr,
    Passthr,   Passthr, ___4___, ___5___, ___6___, Passthr, Passthr, Passthr,  Passthr, Passthr, Passthr, Passthr,
    Passthr,   Passthr, ___7___, ___8___, ___9___, Passthr, Passthr, Passthr,  Passthr, Passthr, Passthr, Passthr,
    Macro(m2), Passthr, Passthr, Passthr, Passthr, Passthr, Passthr, Passthr,  Passthr, Passthr, Passthr, Passthr,
]
];

fn m1() {
    // do stuff
    use super::cc_engine::macros;
    macros::send_string("Test String");
}

fn m2() {
    // do stuff
    use super::cc_engine::macros;
    // add c comment to current line and then go to the end of it
    macros::send_keycode(&CCKeycode::__HOME_);
    macros::send_keycode(&CCKeycode::_SLASH_);
    macros::send_keycode(&CCKeycode::_SLASH_);
    macros::send_keycode(&CCKeycode::__END__);
}

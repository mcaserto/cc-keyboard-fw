// File: keymap.rs
// Description: Contains information about the keyboard keymap
use crate::cc_engine::keycodes::CCKeycode::{self, *};

pub const ROWS: usize = 4;
pub const COLUMNS: usize = 12;
pub const NUM_LAYERS: usize = 3;

#[allow(non_camel_case_types)]
type CCLayer = [CCKeycode; ROWS * COLUMNS];
type CCKeymap = [CCLayer; NUM_LAYERS];

#[rustfmt::skip]
pub const KEYMAP: CCKeymap = [
[
    __TAB__, ___Q___, ___W___, ___E___, ___R___,    ___T___, ___Y___, ___U___,    ___I___, ___O___, ___P___, BACK_SP,
    BACK_SP, ___A___, ___S___, ___D___, ___F___,    ___G___, ___H___, ___J___,    ___K___, ___L___, SEMICLN, _QUOTE_,
    L_SHIFT, ___Z___, ___X___, ___C___, ___V___,    ___B___, ___N___, ___M___,    _COMMA_, __DOT__, _SLASH_, _ENTER_,
    __ESC__, L__CTRL, L___ALT, L___GUI, LAYER(1),  _SPACE_, _SPACE_, LAYER(2),  _LEFT__, _DOWN__, _RIGHT_, __UP___,
],
[
    ___1___,   ___2___, ___3___, ___4___, ___5___, ___6___, ___7___, ___8___,  ___9___, ___0___, PASSTHR, PASSTHR,
    PASSTHR,   FUNCT01, FUNCT02, FUNCT03, FUNCT04, FUNCT05, FUNCT06, PASSTHR,  PASSTHR, PASSTHR, PASSTHR, PASSTHR,
    PASSTHR,   FUNCT07, FUNCT08, FUNCT09, FUNCT10, FUNCT11, FUNCT12, PASSTHR,  PASSTHR, PASSTHR, PASSTHR, PASSTHR,
    MACRO(m1), PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR,  PASSTHR, PASSTHR, PASSTHR, PASSTHR,
],
[
    ___1___,   ___2___, ___3___, ___4___, ___5___, ___6___, ___7___, ___8___,  ___9___, ___0___, PASSTHR, PASSTHR,
    PASSTHR,   PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR,  PASSTHR, PASSTHR, PASSTHR, PASSTHR,
    PASSTHR,   PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR,  PASSTHR, PASSTHR, PASSTHR, PASSTHR,
    MACRO(m2), PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR, PASSTHR,  PASSTHR, PASSTHR, PASSTHR, PASSTHR,
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

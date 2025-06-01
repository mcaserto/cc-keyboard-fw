// File: keymap.rs
// Description: Contains information about the keyboard keymap
use crate::cc_engine::keycodes::Hid::*;
use crate::cc_engine::keycodes::Key::{self, Hid, Layer, Macro, Mod, Pass, DT, MT};
use crate::cc_engine::keycodes::{self, Mod::*};

pub const ROWS: u8 = 4;
pub const COLUMNS: u8 = 12;
pub const NUM_LAYERS: u8 = 3;

// other consts
pub const MOD_TAP_THRESHOLD: u64 = 250; // value in ms
pub const TAP_TAP_THRESHOLD: u64 = 150;

#[allow(non_camel_case_types)]
type KbLayer = [Key; ROWS as usize * COLUMNS as usize];
type Keymap = [KbLayer; NUM_LAYERS as usize];

#[rustfmt::skip]
pub const KEYMAP: Keymap = [
[
    Hid(Tab),    Hid(Q),     Hid(W),    Hid(E),    Hid(R),       Hid(T),     Hid(Y),     Hid(U),       Hid(I),    Hid(O),    Hid(P),       Hid(Backsp),
    Hid(Backsp), Hid(A),     MT(Lalt, S),    MT(Lctrl, D),    MT(Lshft, F), Hid(G),     Hid(H),     MT(Lshft, J), MT(Lctrl, K),    MT(Lalt, L),    Hid(Semicln), Hid(Quote),
    Mod(Lshft),  Hid(Z),     Hid(X),    Hid(C),    Hid(V),       Hid(B),     Hid(N),     Hid(M),       Hid(Com),  Hid(Dot),  Hid(Slash),   Hid(Enter),
    Hid(Esc),    Mod(Lctrl), Mod(Lalt), Mod(Lgui), Layer(1),     Hid(Space), Hid(Space), Layer(2),     Hid(Left), Hid(Down), Hid(Right),   Hid(Up),
],
[
    Pass,      Pass,     Pass,     Pass,     Pass,     Pass,     Pass,      Pass,      Pass,    Pass,       Pass, Pass,
    Pass,      Hid(F01), Hid(F02), Hid(F03), Hid(F04), Hid(F05), Hid(Left), Hid(Down), Hid(Up), Hid(Right), Pass, Pass,
    Pass,      Hid(F07), Hid(F08), Hid(F09), Hid(F10), Hid(F11), Hid(F12),  Pass,      Pass,    Pass,       Pass, Pass,
    Macro(m1), Pass,     Pass,     Pass,     Pass,     Pass,     Pass,      Pass,      Pass,    Pass,       Pass, Pass,
],
[
    Pass,      Hid(N0), Hid(N1), Hid(N2), Hid(N3), Pass, Pass, Pass,  Pass, Pass, Pass, Pass,
    Pass,      Pass,    Hid(N4), Hid(N5), Hid(N6), Pass, Pass, Pass,  Pass, Pass, Pass, Pass,
    Pass,      Pass,    Hid(N7), Hid(N8), Hid(N9), Pass, Pass, Pass,  Pass, Pass, Pass, Pass,
    Macro(m2), Pass,    Pass,    Pass,    Pass,    Pass, Pass, Pass,  Pass, Pass, Pass, Pass,
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
    macros::send_keycode(&keycodes::Hid::Home);
    macros::send_keycode(&keycodes::Hid::Slash);
    macros::send_keycode(&keycodes::Hid::Slash);
    macros::send_keycode(&keycodes::Hid::End);
}

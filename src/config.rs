// File: keymap.rs
// Description: Contains information about the keyboard keymap
use crate::cc_engine::keycodes::Hid::*;
#[allow(unused_imports)]
use crate::cc_engine::keycodes::Key::{self, Ctrl, Hid, Layer, Macro, Mod, Pass, Shft, DT, MT};
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
    Hid(Tab),   Hid(Q),     Hid(W),    Hid(E),    Hid(R),   Hid(T),     Hid(Y),     Hid(U),   Hid(I),    Hid(O),    Hid(P),       Hid(Backsp),
    Hid(Esc),   Hid(A),     Hid(S),    Hid(D),    Hid(F),   Hid(G),     Hid(H),     Hid(J),   Hid(K),    Hid(L),    Hid(Semicln), Hid(Quote),
    Mod(Lshft), Hid(Z),     Hid(X),    Hid(C),    Hid(V),   Hid(B),     Hid(N),     Hid(M),   Hid(Com),  Hid(Dot),  Hid(Slash),   Hid(Enter),
    Pass,       Mod(Lctrl), Mod(Lalt), Mod(Lgui), Layer(1), Hid(Space), Hid(Space), Layer(2), Hid(Left), Hid(Down), Hid(Up),      Hid(Right),
],
[
    Shft(AccTil), Shft(N1), Shft(N2), Shft(N3), Shft(N4), Shft(N5), Shft(N6), Shft(N7),   Shft(N8), Shft(N9),     Shft(N0),     Hid(Backsp),
    Hid(Del),     Hid(F01), Ctrl(S),  Hid(F03), Hid(F04), Hid(F05), Hid(F06), Shft(Dash), Shft(Eq), Shft(LBrace), Shft(RBrace), Shft(BSlsh),
    Pass,         Hid(F07), Hid(F08), Hid(F09), Hid(F10), Hid(F11), Hid(F12), Pass,       Pass,     Hid(Home),    Hid(End),     Pass,
    Macro(m1),    Pass,     Pass,     Pass,     Pass,     Pass,     Pass,     Pass,       Pass,     Pass,         Pass,         Pass,
],
[
    Hid(AccTil), Hid(N1),  Hid(N2),  Hid(N3),  Hid(N4),  Hid(N5),  Hid(N6),  Hid(N7),   Hid(N8),  Hid(N9),     Hid(N0),     Hid(Backsp),
    Pass,        Hid(F01), Hid(F02), Hid(F03), Hid(F04), Hid(F05), Hid(F06), Hid(Dash), Hid(Eq),  Hid(LBrace), Hid(RBrace), Hid(BSlsh),
    Pass,        Hid(F07), Hid(F08), Hid(F09), Hid(F10), Hid(F11), Hid(F12), Pass,      Pass,     Hid(PgUp),   Hid(PgDn),   Pass,
    Macro(m2),   Pass,     Pass,     Pass,     Pass,     Pass,     Pass,     Pass,      Pass,     Pass,        Pass,        Pass,
]
];

fn m1() {
    // do stuff
    use super::cc_engine::macros;
    let test_text = "Lorem ipsum dolor sit amet. Eum dignissimos optio a aperiam exercitationem eos totam deserunt. Sed nisi distinctio ut quia ducimus quo architecto corrupti ut nisi natus sed atque molestiae id molestias labore ut numquam quas.

Eos eveniet quod ab veritatis dolor ut omnis ratione. Eos tempora accusamus quo aliquam galisum id dolorem sunt!";
    macros::send_string(test_text);
}

fn m2() {
    // do stuff
    use super::cc_engine::macros;
    // add c comment to current line and then go to the end of it
    macros::send_keycode(&keycodes::Hid::Home);
    macros::send_keycode(&keycodes::Hid::Slash);
    macros::send_keycode(&keycodes::Hid::Slash);
    macros::send_keycode(&keycodes::Hid::Space);
    macros::send_keycode(&keycodes::Hid::End);
}

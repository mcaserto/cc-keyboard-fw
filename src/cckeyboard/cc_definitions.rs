// keyboard keys
use usbd_hid::descriptor::KeyboardUsage;

#[repr(u8)]
#[allow(unused)]
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub enum Code {
    A = KeyboardUsage::KeyboardAa as u8,
    B = KeyboardUsage::KeyboardBb as u8,
    C = KeyboardUsage::KeyboardCc as u8,
    D = KeyboardUsage::KeyboardDd as u8,
    E = KeyboardUsage::KeyboardEe as u8,
    F = KeyboardUsage::KeyboardFf as u8,
    G = KeyboardUsage::KeyboardGg as u8,
    H = KeyboardUsage::KeyboardHh as u8,
    I = KeyboardUsage::KeyboardIi as u8,
    J = KeyboardUsage::KeyboardJj as u8,
    K = KeyboardUsage::KeyboardKk as u8,
    L = KeyboardUsage::KeyboardLl as u8,
    M = KeyboardUsage::KeyboardMm as u8,
    N = KeyboardUsage::KeyboardNn as u8,
    O = KeyboardUsage::KeyboardOo as u8,
    P = KeyboardUsage::KeyboardPp as u8,
    Q = KeyboardUsage::KeyboardQq as u8,
    R = KeyboardUsage::KeyboardRr as u8,
    S = KeyboardUsage::KeyboardSs as u8,
    T = KeyboardUsage::KeyboardTt as u8,
    U = KeyboardUsage::KeyboardUu as u8,
    V = KeyboardUsage::KeyboardVv as u8,
    W = KeyboardUsage::KeyboardWw as u8,
    X = KeyboardUsage::KeyboardXx as u8,
    Y = KeyboardUsage::KeyboardYy as u8,
    Z = KeyboardUsage::KeyboardZz as u8,
    C1 = KeyboardUsage::Keyboard1Exclamation as u8,
    C2 = KeyboardUsage::Keyboard2At as u8,
    C3 = KeyboardUsage::Keyboard3Hash as u8,
    C4 = KeyboardUsage::Keyboard4Dollar as u8,
    C5 = KeyboardUsage::Keyboard5Percent as u8,
    C6 = KeyboardUsage::Keyboard6Caret as u8,
    C7 = KeyboardUsage::Keyboard7Ampersand as u8,
    C8 = KeyboardUsage::Keyboard8Asterisk as u8,
    C9 = KeyboardUsage::Keyboard9OpenParens as u8,
    C0 = KeyboardUsage::Keyboard0CloseParens as u8,
    Ret = KeyboardUsage::KeyboardEnter as u8,
    Esc = KeyboardUsage::KeyboardEscape as u8,
    Backspace = KeyboardUsage::KeyboardBackspace as u8,
    Tab = KeyboardUsage::KeyboardTab as u8,
    Space = KeyboardUsage::KeyboardSpacebar as u8,
    Scln = KeyboardUsage::KeyboardSemiColon as u8,
    Quote = KeyboardUsage::KeyboardSingleDoubleQuote as u8,
    Comma = KeyboardUsage::KeyboardCommaLess as u8,
    Period = KeyboardUsage::KeyboardPeriodGreater as u8,
    Slash = KeyboardUsage::KeyboardSlashQuestion as u8,
    LeftArrow = KeyboardUsage::KeyboardLeftArrow as u8,
    RightArrow = KeyboardUsage::KeyboardRightArrow as u8,
    UpArrow = KeyboardUsage::KeyboardUpArrow as u8,
    DownArrow = KeyboardUsage::KeyboardDownArrow as u8,
    Tilde = KeyboardUsage::KeyboardBacktickTilde as u8,
    F1 = KeyboardUsage::KeyboardF1 as u8,
    F2 = KeyboardUsage::KeyboardF2 as u8,
    F3 = KeyboardUsage::KeyboardF3 as u8,
    F4 = KeyboardUsage::KeyboardF4 as u8,
    F5 = KeyboardUsage::KeyboardF5 as u8,
    F6 = KeyboardUsage::KeyboardF6 as u8,
    F7 = KeyboardUsage::KeyboardF7 as u8,
    F8 = KeyboardUsage::KeyboardF8 as u8,
    F9 = KeyboardUsage::KeyboardF9 as u8,
    F10 = KeyboardUsage::KeyboardF10 as u8,
    F11 = KeyboardUsage::KeyboardF11 as u8,
    F12 = KeyboardUsage::KeyboardF12 as u8,
    Home = KeyboardUsage::KeyboardHome as u8,
    End = KeyboardUsage::KeyboardEnd as u8,
    PageUp = KeyboardUsage::KeyboardPageUp as u8,
    PageDown = KeyboardUsage::KeyboardPageDown as u8,
    Passthrough = 0x00FF, // Map to keys on extra layers that you want to act as your base layer keys
}

#[repr(u8)]
#[allow(unused)]
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub enum Mod {
    // modifier keys
    Shft = 0x02,
    Ctrl = 0x01,
    Alt = 0x04,
    Gui = 0x08,
}

#[repr(u8)]
#[allow(unused)]
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub enum CCKey {
    Key(Code),  // normal key
    Mod(Mod),   // modifier key
    Lay(usize), // layer shift
}

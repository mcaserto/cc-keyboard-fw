// File: keycodes.rs
// Description: Contains all valid keycodes for cc_engine keyboard firmware
#[repr(u8)]
#[allow(dead_code)]
#[allow(unpredictable_function_pointer_comparisons)]    // needed for Macro
#[derive(Copy, Clone, PartialEq)]
pub enum Key {
    Hid(Hid),     // standard keycodes
    Mod(Mod),     // modifier keycodes
    Media(Media), // media keys
    Sys(Sys),     // system commands
    Shft(Hid),
    //Snap

    // custom codes begin
    Layer(u8),    // layer shift
    Pass,         // pass through
    Macro(fn()),  // macro
    MT(Mod, Hid), // mod tap (modifier, tap keycode)
    DT(Hid, Hid), // double tap (!TODO)
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Hid {
    Nop = 0x00,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    N1 = 0x1E,
    N2 = 0x1F,
    N3 = 0x20,
    N4 = 0x21,
    N5 = 0x22,
    N6 = 0x23,
    N7 = 0x24,
    N8 = 0x25,
    N9 = 0x26,
    N0 = 0x27,
    Enter = 0x28,
    Esc = 0x29,
    Backsp = 0x2A,
    Tab = 0x2B,

    Space = 0x2C,

    Semicln = 0x33,
    Quote = 0x34,

    Com = 0x36,
    Dot = 0x37,
    Slash = 0x38,

    F01 = 0x3A,
    F02 = 0x3B,
    F03 = 0x3C,
    F04 = 0x3D,
    F05 = 0x3E,
    F06 = 0x3F,
    F07 = 0x40,
    F08 = 0x41,
    F09 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    Print = 0x46,
    ScrLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PgUp = 0x4B,
    Del = 0x4C,
    End = 0x4D,
    PgDn = 0x4E,
    Right = 0x4F,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Mod {
    Lctrl = 0x01,
    Lshft = 0x02,
    Lalt = 0x04,
    Lgui = 0x08,
    Rctrl = 0x10,
    Rshft = 0x20,
    Ralt = 0x40,
    Rgui = 0x80,
}

#[repr(u16)]
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Media {
    Lctrl = 0x01,
    Lshft = 0x02,
    Lalt = 0x04,
    Lgui = 0x08,
    Rctrl = 0x10,
    Rshft = 0x20,
    Ralt = 0x40,
    Rgui = 0x80,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Sys {
    PowerDown = 0x81,
    Sleep = 0x82,
    WakeUp = 0x83,
    ContextMenu = 0x84,
    MainMenu = 0x85,
    AppMenu = 0x86,
    MenuHelp = 0x87,
    MenuExit = 0x88,
    MenuSelect = 0x89,
    MenuRight = 0x8A,
    MenuLeft = 0x8B,
    MenuUp = 0x8C,
    MenuDown = 0x8D,
    ColdRestart = 0x8E,
    WarmRestart = 0x8F,
    DpadUp = 0x90,
    DpadDown = 0x91,
    DpadRight = 0x92,
    DpadLeft = 0x93,
    SystemFunctionShift = 0x97,
    SystemFunctionShiftLock = 0x98,
    SystemDismissNotification = 0x9A,
    SystemDoNotDisturb = 0x9B,
    Dock = 0xA0,
    Undock = 0xA1,
    Setup = 0xA2,
    Break = 0xA3,
    DebuggerBreak = 0xA4,
    ApplicationBreak = 0xA5,
    ApplicationDebuggerBreak = 0xA6,
    SpeakerMute = 0xA7,
    Hibernate = 0xA8,
    DisplayInvert = 0xB0,
    DisplayInternal = 0xB1,
    DisplayExternal = 0xB2,
    DisplayBoth = 0xB3,
    DisplayDual = 0xB4,
    DisplayToggleInternalExternal = 0xB5,
    DisplaySwapPrimarySecondary = 0xB6,
    DisplayLcdAutoscale = 0xB7,
    // Use this reserved value to represent all reserved keys / invalid values
    Reserved = 0xB8,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Mouse {
    Lctrl = 0x01,
    Lshft = 0x02,
    Lalt = 0x04,
    Lgui = 0x08,
    Rctrl = 0x10,
    Rshft = 0x20,
    Ralt = 0x40,
    Rgui = 0x80,
}

impl From<char> for Hid {
    fn from(value: char) -> Self {
        match value {
            'a' => Hid::A,
            'A' => Hid::A,
            'b' => Hid::B,
            'B' => Hid::B,
            'c' => Hid::C,
            'C' => Hid::C,
            'd' => Hid::D,
            'D' => Hid::D,
            'e' => Hid::E,
            'E' => Hid::E,
            'f' => Hid::F,
            'F' => Hid::F,
            'g' => Hid::G,
            'G' => Hid::G,
            'h' => Hid::H,
            'H' => Hid::H,
            'i' => Hid::I,
            'I' => Hid::I,
            'j' => Hid::J,
            'J' => Hid::J,
            'k' => Hid::K,
            'K' => Hid::K,
            'l' => Hid::L,
            'L' => Hid::L,
            'm' => Hid::M,
            'M' => Hid::M,
            'n' => Hid::N,
            'N' => Hid::N,
            'o' => Hid::O,
            'O' => Hid::O,
            'p' => Hid::P,
            'P' => Hid::P,
            'q' => Hid::Q,
            'Q' => Hid::Q,
            'r' => Hid::R,
            'R' => Hid::R,
            's' => Hid::S,
            'S' => Hid::S,
            't' => Hid::T,
            'T' => Hid::T,
            'u' => Hid::U,
            'U' => Hid::U,
            'v' => Hid::V,
            'V' => Hid::V,
            'w' => Hid::W,
            'W' => Hid::W,
            'x' => Hid::X,
            'X' => Hid::X,
            'y' => Hid::Y,
            'Y' => Hid::Y,
            'z' => Hid::Z,
            'Z' => Hid::Z,
            '0' => Hid::N0,
            '1' => Hid::N1,
            '2' => Hid::N2,
            '3' => Hid::N3,
            '4' => Hid::N4,
            '5' => Hid::N5,
            '6' => Hid::N6,
            '7' => Hid::N7,
            '8' => Hid::N8,
            '9' => Hid::N9,
            '.' => Hid::Dot,
            ' ' => Hid::Space,
            _ => Hid::Esc,
        }
    }
}

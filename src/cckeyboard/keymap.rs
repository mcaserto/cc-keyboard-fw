use super::cc_definitions::{CCKey, Code, Mod};

// Keyboard matrix size
pub const COLUMNS: usize = 12;
pub const ROWS: usize = 4;
pub const LAYERS: usize = 3;

// keymap is an array of keymaps
type CcKeymapLayer = [[CCKey; COLUMNS]; ROWS];
type CcKeymap = [CcKeymapLayer; LAYERS];

// The keyboard matrix that includes all the layers
pub const KEYMAP: CcKeymap = [BASE_LAYER, LOWER_LAYER, RAISE_LAYER];

const BASE_LAYER: CcKeymapLayer = [
    [
        CCKey::Key(Code::Tab),
        CCKey::Key(Code::Q),
        CCKey::Key(Code::W),
        CCKey::Key(Code::E),
        CCKey::Key(Code::R),
        CCKey::Key(Code::T),
        CCKey::Key(Code::Y),
        CCKey::Key(Code::U),
        CCKey::Key(Code::I),
        CCKey::Key(Code::O),
        CCKey::Key(Code::P),
        CCKey::Key(Code::Backspace),
    ],
    [
        CCKey::Key(Code::Backspace),
        CCKey::Key(Code::A),
        CCKey::Key(Code::S),
        CCKey::Key(Code::D),
        CCKey::Key(Code::F),
        CCKey::Key(Code::G),
        CCKey::Key(Code::H),
        CCKey::Key(Code::J),
        CCKey::Key(Code::K),
        CCKey::Key(Code::L),
        CCKey::Key(Code::Scln),
        CCKey::Key(Code::Quote),
    ],
    [
        CCKey::Mod(Mod::Shft),
        CCKey::Key(Code::Z),
        CCKey::Key(Code::X),
        CCKey::Key(Code::C),
        CCKey::Key(Code::V),
        CCKey::Key(Code::B),
        CCKey::Key(Code::N),
        CCKey::Key(Code::M),
        CCKey::Key(Code::Comma),
        CCKey::Key(Code::Period),
        CCKey::Key(Code::Slash),
        CCKey::Key(Code::Ret),
    ],
    [
        CCKey::Key(Code::Esc),
        CCKey::Mod(Mod::Ctrl),
        CCKey::Mod(Mod::Alt),
        CCKey::Mod(Mod::Gui),
        CCKey::Lay(1),
        CCKey::Key(Code::Space),
        CCKey::Key(Code::Space),
        CCKey::Lay(2),
        CCKey::Key(Code::LeftArrow),
        CCKey::Key(Code::DownArrow),
        CCKey::Key(Code::RightArrow),
        CCKey::Key(Code::UpArrow),
    ],
];

const LOWER_LAYER: CcKeymapLayer = [
    [
        CCKey::Key(Code::C1),
        CCKey::Key(Code::C2),
        CCKey::Key(Code::C3),
        CCKey::Key(Code::C4),
        CCKey::Key(Code::C5),
        CCKey::Key(Code::C6),
        CCKey::Key(Code::C7),
        CCKey::Key(Code::C8),
        CCKey::Key(Code::C9),
        CCKey::Key(Code::C0),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
    ],
    [
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::F1),
        CCKey::Key(Code::F2),
        CCKey::Key(Code::F3),
        CCKey::Key(Code::F4),
        CCKey::Key(Code::F5),
        CCKey::Key(Code::F6),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
    ],
    [
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::F7),
        CCKey::Key(Code::F8),
        CCKey::Key(Code::F9),
        CCKey::Key(Code::F10),
        CCKey::Key(Code::F11),
        CCKey::Key(Code::F12),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
    ],
    [
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
    ],
];

const RAISE_LAYER: CcKeymapLayer = [
    [
        CCKey::Key(Code::Tilde),
        CCKey::Key(Code::C1),
        CCKey::Key(Code::C2),
        CCKey::Key(Code::C3),
        CCKey::Key(Code::C4),
        CCKey::Key(Code::C5),
        CCKey::Key(Code::C6),
        CCKey::Key(Code::C7),
        CCKey::Key(Code::C8),
        CCKey::Key(Code::C9),
        CCKey::Key(Code::C0),
        CCKey::Key(Code::Passthrough),
    ],
    [
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::F1),
        CCKey::Key(Code::F2),
        CCKey::Key(Code::F3),
        CCKey::Key(Code::F4),
        CCKey::Key(Code::F5),
        CCKey::Key(Code::F6),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Quote), // change to minus?
    ],
    [
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::F7),
        CCKey::Key(Code::F8),
        CCKey::Key(Code::F9),
        CCKey::Key(Code::F10),
        CCKey::Key(Code::F11),
        CCKey::Key(Code::F12),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
    ],
    [
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Passthrough),
        CCKey::Key(Code::Home),
        CCKey::Key(Code::PageDown),
        CCKey::Key(Code::PageUp),
        CCKey::Key(Code::End),
    ],
];

use super::cc_definitions::{CCKey, Code, Mod};

// Keyboard matrix size
pub const COLUMNS: usize = 12;
pub const ROWS: usize = 4;

// enum Layer {
//     Base,
//     Upper,
//     Lower,
// }

// define the keyboard matrix
pub const KEYMAP: [[CCKey; COLUMNS]; ROWS] = [
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
        CCKey::Key(Code::SCLN),
        CCKey::Key(Code::QUOTE),
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
        CCKey::Mod(Mod::CTRL),
        CCKey::Mod(Mod::ALT),
        CCKey::Mod(Mod::GUI),
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

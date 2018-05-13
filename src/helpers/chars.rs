#[derive(Debug, Clone)]
enum CharSetType {
    Unicode,
    Nerd,
}

#[derive(Debug, Clone)]
pub struct CharSet {
    char_type: CharSetType,
}

use self::CharSetType::*;

impl CharSet {
    pub fn unicode() -> CharSet {
        CharSet {
            char_type: CharSetType::Unicode,
        }
    }

    pub fn nerd() -> CharSet {
        CharSet {
            char_type: CharSetType::Nerd,
        }
    }

    pub fn volume(&self) -> char {
        match self.char_type {
            Unicode => '\u{1F50A}',
            Nerd => '\u{f028}',
        }
    }

    pub fn volume_off(&self) -> char {
        match self.char_type {
            Unicode => '\u{1F508}',
            Nerd => '\u{f026}',
        }
    }
}

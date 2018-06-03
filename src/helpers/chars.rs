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

    pub fn paper_clip(&self) -> char {
        // Alternates:
        // Paperclip
        // Unicode => '\u{1f4ce}', // 📎 and
        // Nerd => '\u{f8e1}' or '\u{f0c6}' or '\u{f565}'
        match self.char_type {
            Unicode => '\u{1f4f7}', // 📷
            Nerd => '\u{f5ff}',
        }
    }
}

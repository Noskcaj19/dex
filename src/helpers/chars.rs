#[cfg(feature = "nerd_font")]
pub use self::nerd::*;
#[cfg(not(feature = "nerd_font"))]
pub use self::unicode::*;

#[allow(dead_code)]
mod nerd {
    pub const VOLUME: char = '\u{f028}';
    pub const VOLUME_OFF: char = '\u{f026}';
}

#[allow(dead_code)]
mod unicode {
    pub const VOLUME: char = '\u{1F50A}';
    pub const VOLUME_OFF: char = '\u{1F508}';
}

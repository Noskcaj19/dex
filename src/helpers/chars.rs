#[cfg(feature = "nerd_font")]
pub use self::nerd::*;
#[cfg(not(feature = "nerd_font"))]
pub use self::unicode::*;

#[allow(dead_code)]
mod nerd {
    pub const VOLUME: char = '\u{f028}';
}

#[allow(dead_code)]
mod unicode {
    pub const VOLUME: char = '\u{1F50A}';
}

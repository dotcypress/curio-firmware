#![allow(non_upper_case_globals)]

use klaptik::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppIcon {
    Curio = 0,
    Send = 1,
    Scan = 2,
    Replay = 3,
    Config = 4,
    Sleep = 5,
    Backlight = 6,
    About = 7,
}

impl Into<Glyph> for AppIcon {
    fn into(self) -> Glyph {
        self as Glyph
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppIconSmall {
    Address = 0,
    Command = 1,
}

impl Into<Glyph> for AppIconSmall {
    fn into(self) -> Glyph {
        self as Glyph
    }
}

pub const Icon16: RomSprite = RomSprite::new(
    Glyphs::Sequential(8),
    Size::new(16, 16),
    include_bytes!("icons_16.bin"),
);

pub const MenuMedium: RomSprite = RomSprite::new(
    Glyphs::Sequential(8),
    Size::new(56, 16),
    include_bytes!("menu_medium.bin"),
);

pub const MenuSmall: RomSprite = RomSprite::new(
    Glyphs::Sequential(5),
    Size::new(56, 8),
    include_bytes!("menu_small.bin"),
);

pub const MenuLarge: RomSprite = RomSprite::new(
    Glyphs::Sequential(8),
    Size::new(104, 32),
    include_bytes!("menu_large.bin"),
);

pub const Font24: RomSprite = RomSprite::new(
    Glyphs::Alphabet(b"0123456789ABCDEFsx?%. "),
    Size::new(16, 24),
    include_bytes!("font_24.bin"),
);

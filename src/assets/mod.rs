#![allow(non_upper_case_globals)]

use klaptik::*;

pub type MenuIcon = Icon<MenuItem>;
pub type SubMenuIcon = Icon<SubMenuItem>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    Curio = 0,
    Send = 1,
    Scan = 2,
    Replay = 3,
    Config = 4,
    Sleep = 5,
    Backlight = 6,
    About = 7,
}

impl From<MenuItem> for Glyph {
    fn from(item: MenuItem) -> Self {
        item as _
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SubMenuItem {
    Address = 0,
    Command = 1,
}

impl From<SubMenuItem> for Glyph {
    fn from(item: SubMenuItem) -> Self {
        item as _
    }
}

pub const IconSprite: Sprite = Sprite::new(
    Glyphs::Sequential(8),
    Size::new(16, 16),
    include_bytes!("icons.bin"),
);

pub const MenuMediumSprite: Sprite = Sprite::new(
    Glyphs::Sequential(8),
    Size::new(56, 16),
    include_bytes!("menu_medium.bin"),
);

pub const MenuLargeSprite: Sprite = Sprite::new(
    Glyphs::Sequential(8),
    Size::new(104, 32),
    include_bytes!("menu_large.bin"),
);

pub const SubMenuSprite: Sprite = Sprite::new(
    Glyphs::Sequential(5),
    Size::new(56, 8),
    include_bytes!("submenu.bin"),
);

pub const AboutSprite: Sprite = Sprite::new(
    Glyphs::Sequential(1),
    Size::new(88, 32),
    include_bytes!("about.bin"),
);

pub const WebsiteSprite: Sprite = Sprite::new(
    Glyphs::Sequential(1),
    Size::new(104, 8),
    include_bytes!("website.bin"),
);

pub const BatterySprite: Sprite = Sprite::new(
    Glyphs::Sequential(5),
    Size::new(16, 16),
    include_bytes!("battery.bin"),
);

pub const FontSprite: Sprite = Sprite::new(
    Glyphs::Alphabet(b"0123456789ABCDEFsx?%. "),
    Size::new(16, 24),
    include_bytes!("font_24.bin"),
);

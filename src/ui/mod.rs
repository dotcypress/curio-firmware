use crate::app::App;
use crate::game::GameUI;
use klaptik::*;

mod menu;
mod widgets;
mod sprites;

pub use menu::*;
pub use widgets::*;
pub use sprites::*;

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

pub enum Asset {
    Icon = 0,
    Font = 1,
    Battery = 2,
    MenuSmall = 3,
    MenuLarge = 4,
    SubMenu = 5,
    About = 6,
    Website = 7,
    Background = 8,
    GameLogo = 9,
    GamePopup = 10,
    GameBoard = 11,
}

impl From<Asset> for SpriteId {
    fn from(asset: Asset) -> Self {
        asset as _
    }
}

widget_mux! {
    Viewport<&App>,
    ViewportNode::MainMenu,
    {
        main_menu: MenuWidget;
        config_menu: MenuWidget;
        scan: ScanWidget;
        send: SendWidget;
        backlight: BacklightWidget;
        sleep_timeout: SleepTimeoutWidget;
        about: AboutWidget;
        game: GameUI;
    },
    |widget: &mut Viewport, state: &App| {
        widget.main_menu.update(&state.main_menu);
        widget.config_menu.update(&state.config_menu);
        widget.game.update(&state.game);
        widget.backlight.update(state.options.backlight);
        widget.sleep_timeout.update(state.options.sleep_timeout);
        widget.scan.update(state);
        widget.send.update(state);
        widget.set_active(state.active_widget);
    }
}

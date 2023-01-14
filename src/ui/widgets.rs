use super::*;
use crate::app::App;
use core::fmt::Write;

widget!(
    Background<()>,
    Tile<Glyph>,
    Asset::Background,
    0,
    Point::zero(),
    Size::new(16, 16),
    16, 8;
);

widget_group! {
    ScanWidget<&App>,
    {
        bg: Background;
        icon: MenuIcon, Asset::Icon, MenuItem::Scan, Point::zero();
        title: MenuIcon, Asset::MenuSmall, MenuItem::Scan, Point::new(24, 0);
        battery: GlyphIcon, Asset::Battery, 0, Point::new(112, 0);
        addr_title: SubMenuIcon, Asset::SubMenu, SubMenuItem::Address, Point::new(12, 24);
        cmd_title: SubMenuIcon, Asset::SubMenu, SubMenuItem::Command, Point::new(72, 24);
        addr: Label<3>, Asset::Font, "   ", Point::new(8, 40), Size::new(16, 24);
        cmd: Label<3>, Asset::Font, "   ", Point::new(72, 40), Size::new(16, 24);
    },
    |widget: &mut ScanWidget, state: &App| {
        widget.battery.update(state.battery_voltage);
        write!(widget.addr, "{: >3}", state.rx_cmd.addr).ok();
        write!(widget.cmd, "{: >3}", state.rx_cmd.cmd).ok();
    }
}

widget_group! {
    SendWidget<&App>,
    {
        bg: Background;
        icon: MenuIcon, Asset::Icon, MenuItem::Send, Point::zero();
        title: MenuIcon, Asset::MenuSmall, MenuItem::Send, Point::new(24, 0);
        battery: GlyphIcon, Asset::Battery, 0, Point::new(112, 0);
        addr_title: SubMenuIcon, Asset::SubMenu, SubMenuItem::Address, Point::new(12, 24);
        cmd_title: SubMenuIcon, Asset::SubMenu, SubMenuItem::Command, Point::new(72, 24);
        addr: Label<3>, Asset::Font, "000", Point::new(8, 40), Size::new(16, 24);
        cmd: Label<3>, Asset::Font, "000", Point::new(72, 40), Size::new(16, 24);
    },
    |widget: &mut SendWidget, state: &App| {
        widget.battery.update(state.battery_voltage);
        write!(widget.addr, "{: >3}", state.tx_cmd.addr).ok();
        write!(widget.cmd, "{: >3}", state.tx_cmd.cmd).ok();
    }
}

widget_group! {
    BacklightWidget<u8>,
    {
        bg: Background;
        icon: MenuIcon, Asset::Icon, MenuItem::Backlight, Point::zero();
        title: MenuIcon, Asset::MenuSmall, MenuItem::Backlight, Point::new(24, 0);
        brightness: Label<4>, Asset::Font, "100%", Point::new(24, 24), Size::new(16, 24);
    },
    |widget: &mut BacklightWidget, brightness: u8| {
        write!(widget.brightness, "{: >3}%",  brightness * 10).unwrap();
    }
}

widget_group! {
    SleepTimeoutWidget<u8>,
    {
        bg: Background;
        icon: MenuIcon, Asset::Icon, MenuItem::Sleep, Point::zero();
        title: MenuIcon, Asset::MenuSmall, MenuItem::Sleep, Point::new(24, 0);
        value: Label<3>, Asset::Font, "10s", Point::new(32, 24), Size::new(16, 24);
    },
    |widget: &mut SleepTimeoutWidget, timeout: u8| {
        write!(widget.value, "{timeout}s").unwrap();
    }
}

widget_group! {
    AboutWidget<&App>,
    {
        bg: Background;
        icon: MenuIcon, Asset::Icon, MenuItem::Curio, Point::zero();
        title: MenuIcon, Asset::MenuSmall, MenuItem::Curio, Point::new(24, 0);
        about: GlyphIcon, Asset::About, 0, Point::new(24, 16);
        website: GlyphIcon, Asset::Website, 0, Point::new(24, 56);
    }
}

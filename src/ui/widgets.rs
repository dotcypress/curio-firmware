use crate::app::App;
use crate::assets::*;
use core::fmt::Write;
use curio_bsp::Display;
use klaptik::*;

widget_group! {
    ScanWidget<&App>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: MenuIcon, IconSprite, MenuItem::Scan, Point::zero();
        title: MenuIcon, MenuMediumSprite, MenuItem::Scan, Point::new(24, 0);
        battery: GlyphIcon, BatterySprite, 0, Point::new(112, 0);
        addr_title: SubMenuIcon, SubMenuSprite, SubMenuItem::Address, Point::new(12, 24);
        cmd_title: SubMenuIcon, SubMenuSprite, SubMenuItem::Command, Point::new(72, 24);
        addr: SpriteLabel<2>, FontSprite, "00", Point::new(16, 40);
        cmd: SpriteLabel<2>, FontSprite, "00", Point::new(80, 40);
    },
    |widget: &mut ScanWidget, state: &App| {
        widget.battery.update(state.battery_voltage);
        write!(widget.addr, "{:0>2}", state.rx_cmd.addr).ok();
        write!(widget.cmd, "{:0>2}", state.rx_cmd.cmd).ok();
    }
}

widget_group! {
    SendWidget<&App>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: MenuIcon, IconSprite, MenuItem::Send, Point::zero();
        title: MenuIcon, MenuMediumSprite, MenuItem::Send, Point::new(24, 0);
        battery: GlyphIcon, BatterySprite, 0, Point::new(112, 0);
        addr_title: SubMenuIcon, SubMenuSprite, SubMenuItem::Address, Point::new(12, 24);
        cmd_title: SubMenuIcon, SubMenuSprite, SubMenuItem::Command, Point::new(72, 24);
        addr: SpriteLabel<2>, FontSprite, "00", Point::new(16, 40);
        cmd: SpriteLabel<2>, FontSprite, "00", Point::new(80, 40);
    },
    |widget: &mut SendWidget, state: &App| {
        widget.battery.update(state.battery_voltage);
        write!(widget.addr, "{:0>2}", state.tx_cmd.addr).ok();
        write!(widget.cmd, "{:0>2}", state.tx_cmd.cmd).ok();
    }
}

widget_group! {
    BacklightWidget<u8>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: MenuIcon, IconSprite, MenuItem::Backlight, Point::zero();
        title: MenuIcon, MenuMediumSprite, MenuItem::Backlight, Point::new(24, 0);
        brightness: SpriteLabel<4>, FontSprite, "100%", Point::new(24, 24);
    },
    |widget: &mut BacklightWidget, brightness: u8| {
        write!(widget.brightness, "{: >3}%",  brightness * 10).unwrap();
    }
}

widget_group! {
    SleepTimeoutWidget<u8>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: MenuIcon, IconSprite, MenuItem::Sleep, Point::zero();
        title: MenuIcon, MenuMediumSprite, MenuItem::Sleep, Point::new(24, 0);
        value: SpriteLabel<3>, FontSprite, "10s", Point::new(32, 24);
    },
    |widget: &mut SleepTimeoutWidget, timeout: u8| {
        write!(widget.value, "{timeout}s").unwrap();
    }
}

widget_group! {
    AboutWidget<&App>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: MenuIcon, IconSprite, MenuItem::Curio, Point::zero();
        title: MenuIcon, MenuMediumSprite, MenuItem::Curio, Point::new(24, 0);
        about: GlyphIcon, AboutSprite, 0, Point::new(24, 16);
        website: GlyphIcon, WebsiteSprite, 0, Point::new(24, 56);
    }
}

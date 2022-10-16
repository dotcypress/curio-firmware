use crate::app::App;
use crate::assets::*;
use core::fmt::Write;
use curio_bsp::Display;
use klaptik::*;

widget_group! {
    BacklightWidget<u8>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: RomIcon<AppIcon>, Icon16, AppIcon::Backlight, Point::zero();
        title: RomIcon<AppIcon>, MenuMedium, AppIcon::Backlight, Point::new(24, 0);
        brightness: TextBox<RomSprite, 4, 4>, Font24, "100%", Point::new(24, 24);
    },
    |widget: &mut BacklightWidget, brightness: u8| {
        write!(widget.brightness, "{: >3}%",  brightness * 10).unwrap();
    }
}

widget_group! {
    SleepTimeoutWidget<u8>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: RomIcon<AppIcon>, Icon16, AppIcon::Sleep, Point::zero();
        title: RomIcon<AppIcon>, MenuMedium, AppIcon::Sleep, Point::new(24, 0);
        value: TextBox<RomSprite, 3, 3>, Font24, "10s", Point::new(32, 24);
    },
    |widget: &mut SleepTimeoutWidget, timeout: u8| {
        write!(widget.value, "{timeout}s").unwrap();
    }
}

widget_group! {
    ScanWidget<&App>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: RomIcon<AppIcon>, Icon16, AppIcon::Scan, Point::zero();
        title: RomIcon<AppIcon>, MenuMedium, AppIcon::Scan, Point::new(24, 0);
        addr_title: RomIcon<AppIconSmall>, MenuSmall, AppIconSmall::Address, Point::new(12, 24);
        addr: TextBox<RomSprite, 2, 2>, Font24, "00", Point::new(16, 40);
        cmd_title: RomIcon<AppIconSmall>, MenuSmall, AppIconSmall::Command, Point::new(72, 24);
        cmd: TextBox<RomSprite, 2, 2>, Font24, "00", Point::new(80, 40);
    },
    |widget: &mut ScanWidget, state: &App| {
        write!(widget.addr, "{:0>2}", state.last_ir_cmd.addr).ok();
        write!(widget.cmd, "{:0>2}", state.last_ir_cmd.cmd).ok();
    }
}

widget_group! {
    SendWidget<&App>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: RomIcon<AppIcon>, Icon16, AppIcon::Send, Point::zero();
        title: RomIcon<AppIcon>, MenuMedium, AppIcon::Send, Point::new(24, 0);
        addr_title: RomIcon<AppIconSmall>, MenuSmall, AppIconSmall::Address, Point::new(12, 24);
        addr: TextBox<RomSprite, 2, 2>, Font24, "00", Point::new(16, 40);
        cmd_title: RomIcon<AppIconSmall>, MenuSmall, AppIconSmall::Command, Point::new(72, 24);
        cmd: TextBox<RomSprite, 2, 2>, Font24, "00", Point::new(80, 40);
    },
    |widget: &mut SendWidget, state: &App| {
        write!(widget.addr, "{:0>2}", state.ir_cmd.addr).ok();
        write!(widget.cmd, "{:0>2}", state.ir_cmd.cmd).ok();
    }
}

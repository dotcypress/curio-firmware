use crate::app::App;
use crate::assets::*;
use core::fmt::Write;
use curio_bsp::protocol::nec::NecCommand;
use curio_bsp::Display;
use klaptik::*;

pub struct Menu<G: Into<Glyph> + 'static> {
    lines: &'static [G],
    cursor: usize,
}

impl<G: Into<Glyph> + Copy> Menu<G> {
    pub fn new(lines: &'static [G]) -> Self {
        Self { lines, cursor: 0 }
    }

    pub fn move_up(&mut self) {
        self.cursor = if self.cursor == 0 {
            self.lines.len() - 1
        } else {
            self.cursor - 1
        }
    }

    pub fn move_down(&mut self) {
        self.cursor = (self.cursor + 1) % self.lines.len();
    }

    pub fn active(&self) -> G {
        self.lines[(self.cursor + 1) % self.lines.len()]
    }
}

widget_group! {
    MenuWindow<&Menu<AppIcon>>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon1: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::new(0, 0);
        line1: RomIcon<AppIcon>, MenuMedium, AppIcon::About, Point::new(24, 0);
        icon2: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::new(2, 24);
        line2: RomIcon<AppIcon>, MenuLarge, AppIcon::About, Point::new(24, 16);
        icon3: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::new(0, 48);
        line3: RomIcon<AppIcon>, MenuMedium, AppIcon::About, Point::new(24, 48);
    },
    |menu: &mut MenuWindow, state: &Menu<AppIcon>| {
        let mut lines = state.lines.iter().cycle().skip(state.cursor);
        let line = lines.next().unwrap();
        menu.icon1.update(*line);
        menu.line1.update(*line);

        let line = lines.next().unwrap();
        menu.icon2.update(*line);
        menu.line2.update(*line);

        let line = lines.next().unwrap();
        menu.icon3.update(*line);
        menu.line3.update(*line);
    }
}

widget_group! {
    ConfigWindow<u8>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: RomIcon<AppIcon>, Icon16, AppIcon::Config, Point::zero();
        title: RomIcon<AppIcon>, MenuMedium, AppIcon::Config, Point::new(24, 0);
        value: TextBox<RomSprite, 6, 6>, Font24, "000000", Point::new(0, 24);
    },
    |nodes: &mut ConfigWindow, state: u8| {
        let state = state * 10;
        write!(nodes.value, "{state: >5}%").unwrap();
    }
}

widget_group! {
    CommandWindow<&NecCommand>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: RomIcon<AppIcon>, Icon16, AppIcon::Config, Point::zero();
        title: RomIcon<AppIcon>, MenuMedium, AppIcon::Config, Point::new(24, 0);
        addr: TextBox<RomSprite, 2, 2>, Font24, "00", Point::new(8, 40);
        cmd: TextBox<RomSprite, 2, 2>, Font24, "00", Point::new(72, 40);
    },
    |nodes: &mut CommandWindow, state: &NecCommand| {
        let addr = state.addr;
        let cmd = state.cmd;
        write!(nodes.addr, "{addr:0>2}").unwrap();
        write!(nodes.cmd, "{cmd:0>2}").unwrap();
    }
}

widget_mux! {
    UI<&App>,
    UiNode::MainMenu,
    {
        main_menu: MenuWindow;
        scan: CommandWindow;
        cfg_menu: MenuWindow;
        sleep_config: ConfigWindow;
        backlight_config: ConfigWindow;
    },
    |ui: &mut UI, state: &App| {
        ui.set_active(state.active);
        ui.backlight_config.update(state.backlight);
        ui.sleep_config.update(state.sleep_timeout);
        ui.main_menu.update(&state.main_menu);
        ui.cfg_menu.update(&state.cfg_menu);
        ui.scan.update(&state.last_cmd);
    }
}

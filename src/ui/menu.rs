use klaptik::*;
use crate::Background;
use super::*;

widget_group! {
    MenuWidget<&Menu>,
    {
        bg: Background;
        icon1: MenuIcon, Asset::Icon, MenuItem::About, Point::new(0, 0);
        icon2: MenuIcon, Asset::Icon, MenuItem::About, Point::new(2, 24);
        icon3: MenuIcon, Asset::Icon, MenuItem::About, Point::new(0, 48);
        line1: MenuIcon, Asset::MenuSmall, MenuItem::About, Point::new(24, 0);
        line2: MenuIcon, Asset::MenuLarge, MenuItem::About, Point::new(24, 16);
        line3: MenuIcon, Asset::MenuSmall, MenuItem::About, Point::new(24, 48);
    },
    |widget: &mut MenuWidget, state: &Menu| {
        let mut lines = state.lines.iter().cycle().skip(state.cursor);
        let line = lines.next().unwrap();
        widget.icon1.update(*line);
        widget.line1.update(*line);

        let line = lines.next().unwrap();
        widget.icon2.update(*line);
        widget.line2.update(*line);

        let line = lines.next().unwrap();
        widget.icon3.update(*line);
        widget.line3.update(*line);
    }
}

pub struct Menu {
    lines: &'static [MenuItem],
    cursor: usize,
}

impl Menu {
    pub fn new(lines: &'static [MenuItem]) -> Self {
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

    pub fn selected(&self) -> MenuItem {
        self.lines[(self.cursor + 1) % self.lines.len()]
    }
}

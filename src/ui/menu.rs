use crate::assets::*;
use curio_bsp::Display;
use klaptik::*;

widget_group! {
    MenuWidget<&Menu<AppIcon>>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon1: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::new(0, 0);
        line1: RomIcon<AppIcon>, MenuMedium, AppIcon::About, Point::new(24, 0);
        icon2: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::new(2, 24);
        line2: RomIcon<AppIcon>, MenuLarge, AppIcon::About, Point::new(24, 16);
        icon3: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::new(0, 48);
        line3: RomIcon<AppIcon>, MenuMedium, AppIcon::About, Point::new(24, 48);
    },
    |widget: &mut MenuWidget, state: &Menu<AppIcon>| {
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

    pub fn selected(&self) -> G {
        self.lines[(self.cursor + 1) % self.lines.len()]
    }
}

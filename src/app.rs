use crate::assets::MenuItem;
use crate::ui::*;
use curio_bsp::protocol::nec::NecCommand;
use klaptik::*;

pub enum AppEvent {
    ButtonA,
    ButtonB,
    ThumbMove(Point),
    IrCommand(NecCommand),
}

pub enum AppRequest {
    SwitchOff,
    SetBrightness(u8),
    TransmitIRCommand(NecCommand),
}

pub struct App {
    pub frame: u8,
    pub backlight: u8,
    pub sleep_timeout: u8,
    pub active_widget: ViewportNode,
    pub tx_cmd: NecCommand,
    pub rx_cmd: NecCommand,
    pub main_menu: Menu,
    pub config_menu: Menu,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let main_menu = Menu::new(&[MenuItem::Config, MenuItem::Scan, MenuItem::Send]);
        let config_menu = Menu::new(&[MenuItem::About, MenuItem::Sleep, MenuItem::Backlight]);
        let cmd = NecCommand {
            addr: 0,
            cmd: 0,
            repeat: false,
        };
        Self {
            main_menu,
            config_menu,
            backlight: 8,
            frame: 0,
            sleep_timeout: 15,
            tx_cmd: cmd,
            rx_cmd: cmd,
            active_widget: ViewportNode::MainMenu,
        }
    }

    pub fn switch_to(&mut self, widget: ViewportNode) {
        self.active_widget = widget;
    }

    pub fn invalidate(&mut self, ui: &mut Viewport) {
        self.frame = self.frame.wrapping_add(1);
        ui.update(self);
    }

    pub fn handle_event(&mut self, ev: AppEvent) -> Option<AppRequest> {
        match self.active_widget {
            ViewportNode::MainMenu => match ev {
                AppEvent::ButtonA => match self.main_menu.selected() {
                    MenuItem::Config => self.switch_to(ViewportNode::ConfigMenu),
                    MenuItem::Scan => self.switch_to(ViewportNode::Scan),
                    MenuItem::Send => self.switch_to(ViewportNode::Send),
                    _ => {}
                },
                AppEvent::ThumbMove(p) if p.y > 32 => self.main_menu.move_up(),
                AppEvent::ThumbMove(p) if p.y < -32 => self.main_menu.move_down(),
                _ => {}
            },
            ViewportNode::Scan => match ev {
                AppEvent::IrCommand(cmd) => self.rx_cmd = cmd,
                AppEvent::ButtonB => self.switch_to(ViewportNode::MainMenu),
                _ => {}
            },
            ViewportNode::Send => match ev {
                AppEvent::ButtonA => return Some(AppRequest::TransmitIRCommand(self.tx_cmd)),
                AppEvent::ButtonB => self.switch_to(ViewportNode::MainMenu),
                AppEvent::ThumbMove(p) if p.y > 32 => {
                    self.tx_cmd.cmd = self.tx_cmd.cmd.saturating_add(1)
                }
                AppEvent::ThumbMove(p) if p.y < -32 => {
                    self.tx_cmd.cmd = self.tx_cmd.cmd.saturating_sub(1)
                }
                _ => {}
            },
            ViewportNode::ConfigMenu => match ev {
                AppEvent::ButtonA => match self.config_menu.selected() {
                    MenuItem::Backlight => self.switch_to(ViewportNode::Backlight),
                    MenuItem::Sleep => self.switch_to(ViewportNode::SleepTimeout),
                    MenuItem::About => self.switch_to(ViewportNode::About),
                    _ => {}
                },
                AppEvent::ButtonB => self.switch_to(ViewportNode::MainMenu),
                AppEvent::ThumbMove(p) if p.y > 32 => self.config_menu.move_up(),
                AppEvent::ThumbMove(p) if p.y < -32 => self.config_menu.move_down(),

                _ => {}
            },
            ViewportNode::Backlight => match ev {
                AppEvent::ButtonA | AppEvent::ButtonB => self.switch_to(ViewportNode::ConfigMenu),
                AppEvent::ThumbMove(p) if p.y > 32 => {
                    self.backlight = self.backlight.saturating_add(1).clamp(0, 10);
                    return Some(AppRequest::SetBrightness(self.backlight));
                }
                AppEvent::ThumbMove(p) if p.y < -32 => {
                    self.backlight = self.backlight.saturating_sub(1);
                    return Some(AppRequest::SetBrightness(self.backlight));
                }
                _ => {}
            },
            ViewportNode::SleepTimeout => match ev {
                AppEvent::ButtonA | AppEvent::ButtonB => self.switch_to(ViewportNode::ConfigMenu),
                AppEvent::ThumbMove(p) if p.y > 32 => {
                    self.sleep_timeout = self.sleep_timeout.saturating_add(5).clamp(10, 60)
                }
                AppEvent::ThumbMove(p) if p.y < -32 => {
                    self.sleep_timeout = self.sleep_timeout.saturating_sub(5).clamp(10, 60)
                }
                _ => {}
            },
            ViewportNode::About => match ev {
                AppEvent::ButtonA | AppEvent::ButtonB => self.switch_to(ViewportNode::ConfigMenu),
                _ => {}
            },
        }
        None
    }
}

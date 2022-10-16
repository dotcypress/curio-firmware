use crate::{assets::AppIcon, ui::*};
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
    pub ir_cmd: NecCommand,
    pub last_ir_cmd: NecCommand,
    pub main_menu: Menu<AppIcon>,
    pub config_menu: Menu<AppIcon>,
}

impl App {
    pub fn new() -> Self {
        let main_menu = Menu::new(&[
            AppIcon::Config,
            AppIcon::Scan,
            AppIcon::Send,
            AppIcon::Replay,
        ]);
        let config_menu = Menu::new(&[AppIcon::About, AppIcon::Sleep, AppIcon::Backlight]);
        let ir_cmd = NecCommand {
            addr: 0,
            cmd: 3,
            repeat: false,
        };
        Self {
            ir_cmd,
            main_menu,
            config_menu,
            backlight: 8,
            frame: 0,
            sleep_timeout: 15,
            last_ir_cmd: ir_cmd,
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
                    AppIcon::Config => self.switch_to(ViewportNode::ConfigMenu),
                    AppIcon::Scan => self.switch_to(ViewportNode::Scan),
                    AppIcon::Send => self.switch_to(ViewportNode::Send),
                    _ => {}
                },
                AppEvent::ThumbMove(p) => {
                    if p.y > 32 {
                        self.main_menu.move_up();
                    } else if p.y < -32 {
                        self.main_menu.move_down();
                    }
                }
                _ => {}
            },
            ViewportNode::Scan => match ev {
                AppEvent::IrCommand(cmd) => self.last_ir_cmd = cmd,
                AppEvent::ButtonB => self.switch_to(ViewportNode::MainMenu),
                _ => {}
            },
            ViewportNode::Send => match ev {
                AppEvent::ButtonA => return Some(AppRequest::TransmitIRCommand(self.ir_cmd)),
                AppEvent::ButtonB => self.switch_to(ViewportNode::MainMenu),
                AppEvent::ThumbMove(p) => {
                    if p.y > 32 {
                        self.ir_cmd.cmd = self.ir_cmd.cmd.saturating_add(1);
                    } else if p.y < -32 {
                        self.ir_cmd.cmd = self.ir_cmd.cmd.saturating_sub(1);
                    }
                }
                _ => {}
            },
            ViewportNode::ConfigMenu => match ev {
                AppEvent::ButtonA => match self.config_menu.selected() {
                    AppIcon::Backlight => self.switch_to(ViewportNode::Backlight),
                    AppIcon::Sleep => self.switch_to(ViewportNode::SleepTimeout),
                    _ => {}
                },
                AppEvent::ButtonB => self.switch_to(ViewportNode::MainMenu),
                AppEvent::ThumbMove(p) => {
                    if p.y > 32 {
                        self.config_menu.move_up();
                    } else if p.y < -32 {
                        self.config_menu.move_down();
                    }
                }
                _ => {}
            },
            ViewportNode::Backlight => match ev {
                AppEvent::ButtonA | AppEvent::ButtonB => self.switch_to(ViewportNode::ConfigMenu),
                AppEvent::ThumbMove(p) => {
                    if p.y > 32 {
                        self.backlight = self.backlight.saturating_add(1).clamp(0, 10);
                        return Some(AppRequest::SetBrightness(self.backlight));
                    } else if p.y < -32 {
                        self.backlight = self.backlight.saturating_sub(1);
                        return Some(AppRequest::SetBrightness(self.backlight));
                    }
                }
                _ => {}
            },
            ViewportNode::SleepTimeout => match ev {
                AppEvent::ButtonA | AppEvent::ButtonB => self.switch_to(ViewportNode::ConfigMenu),
                AppEvent::ThumbMove(p) => {
                    if p.y > 32 {
                        self.sleep_timeout = self.sleep_timeout.saturating_add(1).clamp(10, 60);
                    } else if p.y < -32 {
                        self.sleep_timeout = self.sleep_timeout.saturating_sub(1).clamp(10, 60);
                    }
                }
                _ => {}
            },
        }
        None
    }
}

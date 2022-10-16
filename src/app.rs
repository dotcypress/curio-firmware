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
    pub active: UiNode,
    pub last_cmd: NecCommand,
    pub main_menu: Menu<AppIcon>,
    pub cfg_menu: Menu<AppIcon>,
}

impl App {
    pub fn new() -> Self {
        let main_menu = Menu::new(&[
            AppIcon::Send,
            AppIcon::Scan,
            AppIcon::Replay,
            AppIcon::Config,
        ]);
        let cfg_menu = Menu::new(&[AppIcon::About, AppIcon::Sleep, AppIcon::Backlight]);
        let cmd = NecCommand {
            addr: 12,
            cmd: 34,
            repeat: false,
        };
        Self {
            frame: 0,
            backlight: 8,
            sleep_timeout: 15,
            active: UiNode::Scan,
            main_menu,
            cfg_menu,
            last_cmd: cmd,
        }
    }

    pub fn invalidate(&mut self, ui: &mut UI) {
        self.frame = self.frame.wrapping_add(1);
        ui.update(self);
    }

    pub fn handle_event(&mut self, ev: AppEvent) -> Option<AppRequest> {
        match self.active {
            UiNode::MainMenu => match ev {
                AppEvent::ButtonA => match self.main_menu.active() {
                    AppIcon::Config => self.active = UiNode::CfgMenu,
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
            UiNode::CfgMenu => match ev {
                AppEvent::ButtonA => match self.cfg_menu.active() {
                    AppIcon::Backlight => self.active = UiNode::BacklightConfig,
                    _ => {}
                },
                AppEvent::ButtonB => self.active = UiNode::MainMenu,
                AppEvent::ThumbMove(p) => {
                    if p.y > 32 {
                        self.cfg_menu.move_up();
                    } else if p.y < -32 {
                        self.cfg_menu.move_down();
                    }
                }
                _ => {}
            },
            UiNode::BacklightConfig => match ev {
                AppEvent::ButtonB => self.active = UiNode::CfgMenu,
                AppEvent::ThumbMove(p) => {
                    if p.y > 32 {
                        self.backlight = (self.backlight + 1).clamp(0, 10);
                        return Some(AppRequest::SetBrightness(self.backlight));
                    } else if p.y < -32 {
                        self.backlight = self.backlight.saturating_sub(1);
                        return Some(AppRequest::SetBrightness(self.backlight));
                    }
                }
                _ => {}
            },
            UiNode::Scan => match ev {
                AppEvent::IrCommand(cmd) => self.last_cmd = cmd,
                AppEvent::ButtonA => {
                    return Some(AppRequest::TransmitIRCommand(NecCommand {
                        addr: 0,
                        cmd: 3,
                        repeat: false,
                    }))
                }
                _ => {}
            },
            UiNode::SleepConfig => todo!(),
        }
        None
    }
}

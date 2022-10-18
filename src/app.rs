use crate::assets::MenuItem;
use crate::ui::*;
use curio_bsp::hal::flash::FlashPage;
use curio_bsp::protocol::nec::NecCommand;
use klaptik::*;

pub enum AppEvent {
    ButtonA,
    ButtonB,
    ThumbMove(Point),
    IrCommand(NecCommand),
}

impl AppEvent {
    pub fn is_user_event(&self) -> bool {
        matches!(self, AppEvent::ButtonA | AppEvent::ButtonB)
    }
}

pub enum AppRequest {
    SwitchOff,
    SetBrightness(u8),
    TransmitIRCommand(NecCommand),
    StoreOptions(Options),
}

pub struct App {
    pub frame: u8,
    pub options: Options,
    pub sleep_timeout: u32,
    pub battery_voltage: Glyph,
    pub active_widget: ViewportNode,
    pub tx_cmd: NecCommand,
    pub rx_cmd: NecCommand,
    pub main_menu: Menu,
    pub config_menu: Menu,
}

impl App {
    pub fn new(options: Options, battery_voltage: u16) -> Self {
        let main_menu = Menu::new(&[MenuItem::Config, MenuItem::Scan, MenuItem::Send]);
        let config_menu = Menu::new(&[MenuItem::About, MenuItem::Sleep, MenuItem::Backlight]);
        let cmd = NecCommand {
            addr: 0,
            cmd: 0,
            repeat: false,
        };

        let battery_voltage = battery_voltage.saturating_sub(2200) / 200;
        let battery_voltage = battery_voltage.clamp(0, 4) as _;

        Self {
            main_menu,
            config_menu,
            battery_voltage,
            options,
            frame: 0,
            tx_cmd: cmd,
            rx_cmd: cmd,
            sleep_timeout: 0,
            active_widget: ViewportNode::MainMenu,
        }
    }

    pub fn switch_to(&mut self, widget: ViewportNode) {
        self.active_widget = widget;
    }

    pub fn tick(&mut self) -> Option<AppRequest> {
        self.frame = self.frame.wrapping_add(1);
        self.sleep_timeout = self.sleep_timeout.wrapping_add(1);
        if self.sleep_timeout / 10 > self.options.sleep_timeout as _ {
            return Some(AppRequest::SwitchOff);
        }
        None
    }

    pub fn handle_event(&mut self, ev: AppEvent) -> Option<AppRequest> {
        if ev.is_user_event() {
            self.sleep_timeout = 0;
        }
        match self.active_widget {
            ViewportNode::MainMenu => match ev {
                AppEvent::ButtonB => return Some(AppRequest::SwitchOff),
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
                AppEvent::ButtonB => self.switch_to(ViewportNode::MainMenu),
                AppEvent::IrCommand(cmd) => {
                    self.rx_cmd = cmd;
                }
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
                AppEvent::ButtonA => {
                    self.switch_to(ViewportNode::ConfigMenu);
                    return Some(AppRequest::StoreOptions(self.options));
                }
                AppEvent::ButtonB => self.switch_to(ViewportNode::ConfigMenu),
                AppEvent::ThumbMove(p) if p.y > 32 => {
                    self.options.backlight = self.options.backlight.saturating_add(1).clamp(0, 10);
                    return Some(AppRequest::SetBrightness(self.options.backlight));
                }
                AppEvent::ThumbMove(p) if p.y < -32 => {
                    self.options.backlight = self.options.backlight.saturating_sub(1);
                    return Some(AppRequest::SetBrightness(self.options.backlight));
                }
                _ => {}
            },
            ViewportNode::SleepTimeout => match ev {
                AppEvent::ButtonA => {
                    self.switch_to(ViewportNode::ConfigMenu);
                    return Some(AppRequest::StoreOptions(self.options));
                }
                AppEvent::ButtonB => self.switch_to(ViewportNode::ConfigMenu),
                AppEvent::ThumbMove(p) if p.y > 32 => {
                    self.options.sleep_timeout =
                        self.options.sleep_timeout.saturating_add(5).clamp(10, 90)
                }
                AppEvent::ThumbMove(p) if p.y < -32 => {
                    self.options.sleep_timeout =
                        self.options.sleep_timeout.saturating_sub(5).clamp(10, 90)
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

#[derive(Clone, Copy)]
pub struct Options {
    pub backlight: u8,
    pub sleep_timeout: u8,
}

impl Options {
    pub const PAGE: FlashPage = FlashPage(31);

    pub fn load() -> Self {
        let opts = unsafe { core::ptr::read(Self::PAGE.to_address() as *const u32) };
        let [_, _, mut backlight, mut sleep_timeout] = opts.to_le_bytes();
        if !(10..=90).contains(&sleep_timeout) {
            sleep_timeout = 90;
            backlight = 8;
        }
        Self {
            backlight,
            sleep_timeout,
        }
    }

    pub fn to_bytes(self) -> [u8; 4] {
        [0, 0, self.backlight, self.sleep_timeout]
    }
}

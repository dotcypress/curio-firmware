use core::ops::Mul;

use crate::assets::MenuItem;
use crate::game::Minesweeper;
use crate::ui::*;
use curio_bsp::hal::flash::FlashPage;
use curio_bsp::protocol::nec::NecCommand;
use curio_bsp::Button;
use klaptik::*;

pub enum AppEvent {
    ClockTick,
    Button(Button),
    IrCommand(NecCommand),
}

pub enum AppRequest {
    SwitchOff,
    SetBrightness(u8),
    TransmitIRCommand(NecCommand),
    StoreOptions(Options),
}

pub struct App {
    pub frame: u8,
    pub sleep_timeout: u32,
    pub battery_voltage: Glyph,
    pub options: Options,
    pub game: Minesweeper,
    pub active_widget: ViewportNode,
    pub tx_cmd: NecCommand,
    pub rx_cmd: NecCommand,
    pub address_edit: bool,
    pub main_menu: Menu,
    pub config_menu: Menu,
}

impl App {
    pub fn new(options: Options, battery_voltage: u16) -> Self {
        let main_menu = Menu::new(&[MenuItem::Scan, MenuItem::Send, MenuItem::Config]);
        let config_menu = Menu::new(&[MenuItem::About, MenuItem::Sleep, MenuItem::Backlight]);
        let cmd = NecCommand {
            addr: 0,
            cmd: 0,
            repeat: false,
        };

        let battery_voltage = battery_voltage.mul(3).saturating_sub(2200) / 200;
        let battery_voltage = battery_voltage.clamp(0, 4) as _;
        let game = Minesweeper::new(8);
        Self {
            main_menu,
            config_menu,
            battery_voltage,
            options,
            game,
            frame: 0,
            tx_cmd: cmd,
            rx_cmd: cmd,
            sleep_timeout: 0,
            address_edit: false,
            active_widget: ViewportNode::MainMenu,
        }
    }

    pub fn switch_to(&mut self, widget: ViewportNode) {
        self.active_widget = widget;
    }

    pub fn handle_event(&mut self, ev: AppEvent) -> Option<AppRequest> {
        match ev {
            AppEvent::ClockTick => {
                self.frame = self.frame.wrapping_add(1);
                self.sleep_timeout = self.sleep_timeout.wrapping_add(1);
                if self.sleep_timeout / 10 > self.options.sleep_timeout as _ {
                    Some(AppRequest::SwitchOff)
                } else {
                    None
                }
            }
            AppEvent::IrCommand(cmd) => {
                self.rx_cmd = cmd;
                None
            }
            AppEvent::Button(btn) => self.handle_button(btn),
        }
    }

    fn handle_button(&mut self, btn: Button) -> Option<AppRequest> {
        self.sleep_timeout = 0;

        match self.active_widget {
            ViewportNode::Game => self.game.button_click(btn),
            ViewportNode::MainMenu => match btn {
                Button::A => match self.main_menu.selected() {
                    MenuItem::Config => self.switch_to(ViewportNode::ConfigMenu),
                    MenuItem::Scan => self.switch_to(ViewportNode::Scan),
                    MenuItem::Send => self.switch_to(ViewportNode::Send),
                    _ => {}
                },
                Button::B => return Some(AppRequest::SwitchOff),
                Button::Up => self.main_menu.move_up(),
                Button::Down => self.main_menu.move_down(),
                _ => {}
            },
            ViewportNode::Scan => match btn {
                Button::A => return Some(AppRequest::TransmitIRCommand(self.rx_cmd)),
                Button::B => self.switch_to(ViewportNode::MainMenu),
                _ => {}
            },
            ViewportNode::Send => match btn {
                Button::A => return Some(AppRequest::TransmitIRCommand(self.tx_cmd)),
                Button::B => self.switch_to(ViewportNode::MainMenu),
                Button::Right => self.address_edit = false,
                Button::Left => self.address_edit = true,
                Button::Up if self.address_edit => {
                    self.tx_cmd.addr = self.tx_cmd.addr.wrapping_add(1)
                }
                Button::Up if !self.address_edit => {
                    self.tx_cmd.cmd = self.tx_cmd.cmd.wrapping_add(1)
                }
                Button::Down if self.address_edit => {
                    self.tx_cmd.addr = self.tx_cmd.addr.wrapping_sub(1);
                }
                Button::Down if !self.address_edit => {
                    self.tx_cmd.cmd = self.tx_cmd.cmd.wrapping_sub(1);
                }
                _ => {}
            },
            ViewportNode::ConfigMenu => match btn {
                Button::A => match self.config_menu.selected() {
                    MenuItem::Backlight => self.switch_to(ViewportNode::Backlight),
                    MenuItem::Sleep => self.switch_to(ViewportNode::SleepTimeout),
                    MenuItem::About => self.switch_to(ViewportNode::About),
                    _ => {}
                },
                Button::B => self.switch_to(ViewportNode::MainMenu),
                Button::Up => self.config_menu.move_up(),
                Button::Down => self.config_menu.move_down(),

                _ => {}
            },
            ViewportNode::Backlight => match btn {
                Button::A => {
                    self.switch_to(ViewportNode::ConfigMenu);
                    return Some(AppRequest::StoreOptions(self.options));
                }
                Button::B => self.switch_to(ViewportNode::ConfigMenu),
                Button::Up => {
                    self.options.backlight = self.options.backlight.saturating_add(1).clamp(0, 10);
                    return Some(AppRequest::SetBrightness(self.options.backlight));
                }
                Button::Down => {
                    self.options.backlight = self.options.backlight.saturating_sub(1);
                    return Some(AppRequest::SetBrightness(self.options.backlight));
                }
                _ => {}
            },
            ViewportNode::SleepTimeout => match btn {
                Button::A => {
                    self.switch_to(ViewportNode::ConfigMenu);
                    return Some(AppRequest::StoreOptions(self.options));
                }
                Button::B => self.switch_to(ViewportNode::ConfigMenu),
                Button::Up => {
                    self.options.sleep_timeout =
                        self.options.sleep_timeout.saturating_add(5).clamp(10, 90)
                }
                Button::Down => {
                    self.options.sleep_timeout =
                        self.options.sleep_timeout.saturating_sub(5).clamp(10, 90)
                }
                _ => {}
            },
            ViewportNode::About => match btn {
                Button::A => self.switch_to(ViewportNode::Game),
                Button::B => self.switch_to(ViewportNode::ConfigMenu),
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
        let opts = unsafe { core::ptr::read(Self::PAGE.to_address() as *const u16) };
        let [backlight, sleep_timeout] = opts.to_le_bytes();
        Self {
            backlight: backlight.clamp(0, 10),
            sleep_timeout: sleep_timeout.clamp(10, 60),
        }
    }

    pub fn into_bytes(self) -> [u8; 2] {
        [self.backlight, self.sleep_timeout]
    }
}

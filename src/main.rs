#![no_std]
#![no_main]
// #![deny(warnings)]

extern crate panic_halt;
extern crate rtic;

mod app;
mod game;
mod ui;

use defmt_rtt as _;

use app::*;
use curio_bsp::hal::flash::WriteErase;
use curio_bsp::hal::gpio::SignalEdge;
use curio_bsp::hal::power::*;
use curio_bsp::hal::rcc::*;
use curio_bsp::hal::timer::Timer;
use curio_bsp::stm32::*;
use curio_bsp::*;
use klaptik::{SpriteDisplay, Widget};
use ui::*;

#[rtic::app(device = stm32, peripherals = true, dispatchers = [CEC])]
mod curio {

    use super::*;

    #[shared]
    struct Shared {
        app: App,
        control: Control,
        display: SpriteDisplay<DisplayController, { SPRITES.len() }>,
        ir: IrTransceiver,
        exti: EXTI,
        i2c: I2cDev,
    }

    #[local]
    struct Local {
        pwr: Power,
        scb: stm32::SCB,
        flash: Option<FLASH>,
        ui: Viewport,
        ui_timer: Timer<stm32::TIM14>,
        render_timer: Timer<stm32::TIM17>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("init");
        let scb = ctx.core.SCB;
        let flash = Some(ctx.device.FLASH);
        let mut exti = ctx.device.EXTI;
        let mut rcc = ctx.device.RCC.constrain();
        let mut pwr = ctx.device.PWR.constrain(&mut rcc);
        pwr.clear_standby_flag();
        pwr.enable_wakeup_lane(WakeUp::Line4, SignalEdge::Falling);

        let Curio {
            mut control,
            mut display,
            i2c,
            ir,
        } = Curio::new(
            ctx.device.ADC,
            ctx.device.GPIOA,
            ctx.device.GPIOB,
            ctx.device.GPIOC,
            ctx.device.TIM1,
            ctx.device.TIM16,
            ctx.device.SPI1,
            ctx.device.I2C1,
            i2c::Config::new(400.kHz()),
            &mut exti,
            &mut rcc,
        );

        let mut ui_timer = ctx.device.TIM14.timer(&mut rcc);
        ui_timer.start(150.millis());
        ui_timer.listen();

        let mut render_timer = ctx.device.TIM17.timer(&mut rcc);
        render_timer.start(100.millis());
        render_timer.listen();

        let options = Options::load();
        display.set_brightness(options.backlight);
        let app = App::new(options, control.battery_voltage());
        let ui = Viewport::new();

        let display = SpriteDisplay::new(display, SPRITES);

        (
            Shared {
                app,
                control,
                display,
                exti,
                i2c,
                ir,
            },
            Local {
                flash,
                ui_timer,
                ui,
                render_timer,
                pwr,
                scb,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = EXTI2_3, shared = [app, control, exti])]
    fn button_click(ctx: button_click::Context) {
        let button_click::SharedResources {
            mut app,
            mut control,
            mut exti,
        } = ctx.shared;

        exti.lock(|exti| {
            exti.unpend(hal::exti::Event::GPIO2);
            exti.unpend(hal::exti::Event::GPIO3);
        });

        if let Some(btn) = control.lock(|ctrl| ctrl.read_buttons()) {
            app.lock(|app| app.handle_event(AppEvent::Button(btn)))
                .map(app_request::spawn);
        }
    }

    #[task(binds = EXTI4_15, shared = [app, exti, ir])]
    fn ir_rx(ctx: ir_rx::Context) {
        let ir_rx::SharedResources {
            mut app,
            mut exti,
            mut ir,
        } = ctx.shared;
        exti.lock(|exti| exti.unpend(hal::exti::Event::GPIO12));

        if let Ok(Some(cmd)) = ir.lock(|ir| ir.event()) {
            app.lock(|app| app.handle_event(AppEvent::IrCommand(cmd)))
                .map(app_request::spawn);
        }
    }

    #[task(binds = TIM14, local = [ui_timer], shared = [app, control])]
    fn ui_timer_tick(ctx: ui_timer_tick::Context) {
        let mut app = ctx.shared.app;
        let mut control = ctx.shared.control;

        if let Some(btn) = control.lock(|ctrl| ctrl.read_dpad()) {
            app.lock(|app| app.handle_event(AppEvent::Button(btn)))
                .map(app_request::spawn);
        }

        ctx.local.ui_timer.clear_irq();
    }

    #[task(binds = TIM16, shared = [ir])]
    fn ir_timer_tick(ctx: ir_timer_tick::Context) {
        let mut ir = ctx.shared.ir;
        ir.lock(|ir| ir.tick());
    }

    #[task(binds = TIM17, local = [ui, render_timer], shared = [app, display])]
    fn render_timer_tick(ctx: render_timer_tick::Context) {
        let ui = ctx.local.ui;
        let render_timer = ctx.local.render_timer;
        let mut app = ctx.shared.app;
        let mut display = ctx.shared.display;

        app.lock(|app| {
            app.handle_event(AppEvent::ClockTick)
                .map(app_request::spawn);
            ui.update(app);
        });
        display.lock(|display| ui.render(display));

        render_timer.clear_irq();
    }

    #[task(capacity = 4, local = [flash, pwr, scb], shared = [i2c, ir, display])]
    fn app_request(ctx: app_request::Context, req: AppRequest) {
        match req {
            AppRequest::SetBrightness(val) => {
                let mut display = ctx.shared.display;
                display.lock(|display| display.canvas().set_brightness(val));
            }
            AppRequest::TransmitIRCommand(cmd) => {
                let mut ir = ctx.shared.ir;
                ir.lock(|ir| ir.send(&cmd));
            }
            AppRequest::SwitchOff => {
                let pwr = ctx.local.pwr;
                pwr.clear_wakeup_flag(WakeUp::Line4);
                pwr.set_mode(PowerMode::LowPower(LowPowerMode::Shutdown));
                ctx.local.scb.set_sleepdeep();
            }
            AppRequest::StoreOptions(options) => {
                if let Some(flash) = ctx.local.flash.take() {
                    hal::cortex_m::interrupt::free(|_| {
                        if let Ok(mut unlocked) = flash.unlock() {
                            unlocked.erase_page(Options::PAGE).ok();
                            let addr = Options::PAGE.to_address();
                            unlocked.write(addr, &options.into_bytes()).ok();
                            *ctx.local.flash = Some(unlocked.lock());
                        }
                    });
                }
            }
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
        }
    }
}

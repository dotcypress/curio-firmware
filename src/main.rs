#![no_std]
#![no_main]
// #![deny(warnings)]

extern crate panic_halt;
extern crate rtic;

mod app;
mod assets;
mod ui;

use defmt_rtt as _;

use app::*;
use curio_bsp::hal::power::*;
use curio_bsp::hal::rcc::*;
use curio_bsp::hal::timer::Timer;
use curio_bsp::*;
use klaptik::Widget;
use ui::*;

#[rtic::app(device = stm32, peripherals = true, dispatchers = [CEC])]
mod curio {
    use super::*;

    #[shared]
    struct Shared {
        app: App,
        control: Control,
        display: Display,
        ir: IrTransceiver,
        i2c: I2cDev,
    }

    #[local]
    struct Local {
        ui: UI,
        scb: stm32::SCB,
        pwr: Power,
        rcc: Rcc,
        ui_timer: Timer<stm32::TIM14>,
        render_timer: Timer<stm32::TIM17>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("init");
        let mut rcc = ctx.device.RCC.constrain();

        let Curio {
            control,
            mut display,
            i2c,
            ir,
        } = Curio::new(
            ctx.device.ADC,
            ctx.device.GPIOA,
            ctx.device.GPIOB,
            ctx.device.GPIOC,
            ctx.device.EXTI,
            ctx.device.TIM1,
            ctx.device.TIM16,
            ctx.device.SPI1,
            ctx.device.I2C1,
            i2c::Config::new(400.kHz()),
            &mut rcc,
        );

        let mut ui_timer = ctx.device.TIM14.timer(&mut rcc);
        ui_timer.start(150.millis());
        ui_timer.listen();

        let mut render_timer = ctx.device.TIM17.timer(&mut rcc);
        render_timer.start(50.millis());
        render_timer.listen();

        let pwr = ctx.device.PWR.constrain(&mut rcc);
        let scb = ctx.core.SCB;

        let app = App::new();
        let ui = UI::new();

        display.set_brightness(8);

        defmt::info!("init done");

        (
            Shared {
                app,
                control,
                display,
                i2c,
                ir,
            },
            Local {
                ui_timer,
                ui,
                render_timer,
                pwr,
                rcc,
                scb,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = EXTI2_3, shared = [app, control])]
    fn button_click(ctx: button_click::Context) {
        let mut app = ctx.shared.app;
        let mut control = ctx.shared.control;

        match control.lock(|ctrl| ctrl.buttons()) {
            (true, false) => app.lock(|app| app.handle_event(AppEvent::ButtonA)),
            (false, true) => app.lock(|app| app.handle_event(AppEvent::ButtonB)),
            _ => None,
        }
        .map(app_request::spawn);
    }

    #[task(binds = TIM14, local = [ui_timer], shared = [app, control])]
    fn ui_timer_tick(ctx: ui_timer_tick::Context) {
        let mut app = ctx.shared.app;
        let mut control = ctx.shared.control;

        let thumb = control.lock(|ctrl| ctrl.thumb());
        app.lock(|app| app.handle_event(AppEvent::ThumbMove(thumb)))
            .map(app_request::spawn);

        ctx.local.ui_timer.clear_irq();
    }

    #[task(binds = TIM16, shared = [app, ir])]
    fn ir_timer_tick(ctx: ir_timer_tick::Context) {
        let mut app = ctx.shared.app;
        let mut ir = ctx.shared.ir;

        if let Ok(Some(cmd)) = ir.lock(|ir| ir.poll()) {
            app.lock(|app| app.handle_event(AppEvent::IrCommand(cmd)))
                .map(app_request::spawn);
        }
    }

    #[task(binds = TIM17, local = [ui, render_timer], shared = [app, display])]
    fn render_timer_tick(ctx: render_timer_tick::Context) {
        let ui = ctx.local.ui;
        let render_timer = ctx.local.render_timer;
        let mut app = ctx.shared.app;
        let mut display = ctx.shared.display;

        app.lock(|app| app.invalidate(ui));
        display.lock(|display| ui.render(display));

        render_timer.clear_irq();
    }

    #[task(local = [pwr, rcc, scb], shared = [i2c, ir, display])]
    fn app_request(ctx: app_request::Context, req: AppRequest) {
        match req {
            AppRequest::SetBrightness(val) => {
                let mut display = ctx.shared.display;
                display.lock(|display| display.set_brightness(val));
                //TODO: save to flash
            }
            AppRequest::TransmitIRCommand(cmd) => {
                let mut ir = ctx.shared.ir;
                ir.lock(|ir| ir.send(&cmd));
            }
            AppRequest::SwitchOff => {
                let pwr = ctx.local.pwr;
                let mut display = ctx.shared.display;
                display.lock(|display| display.power_off());
                //TODO: Slow down clocks to 1MHz
                pwr.set_mode(PowerMode::UltraLowPower(LowPowerMode::StopMode2));
                ctx.local.scb.set_sleepdeep();
            }
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::export::nop();
        }
    }
}

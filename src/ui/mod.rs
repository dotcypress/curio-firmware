use crate::app::App;
use crate::game::GameUI;
use klaptik::*;

mod menu;
mod widgets;

pub use menu::*;
pub use widgets::*;

widget_mux! {
    Viewport<&App>,
    ViewportNode::MainMenu,
    {
        main_menu: MenuWidget;
        config_menu: MenuWidget;
        scan: ScanWidget;
        send: SendWidget;
        backlight: BacklightWidget;
        sleep_timeout: SleepTimeoutWidget;
        about: AboutWidget;
        game: GameUI;
    },
    |widget: &mut Viewport, state: &App| {
        widget.main_menu.update(&state.main_menu);
        widget.config_menu.update(&state.config_menu);
        widget.game.update(&state.game);
        widget.backlight.update(state.options.backlight);
        widget.sleep_timeout.update(state.options.sleep_timeout);
        widget.scan.update(state);
        widget.send.update(state);
        widget.set_active(state.active_widget);
    }
}

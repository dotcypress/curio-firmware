use crate::app::App;
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
  },
  |widget: &mut Viewport, state: &App| {
      widget.main_menu.update(&state.main_menu);
      widget.config_menu.update(&state.config_menu);
      widget.backlight.update(state.backlight);
      widget.sleep_timeout.update(state.sleep_timeout);
      widget.scan.update(state);
      widget.send.update(state);
      widget.set_active(state.active_widget);
  }
}

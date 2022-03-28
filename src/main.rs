mod osxnav_app;
mod osxnav_window;
mod osxnav;
mod draw;

use std::sync::RwLock;
use cacao::macos::App;
use cacao::macos::window::{Window, WindowConfig, WindowStyle};
use crate::osxnav_app::OsxNavApp;
use crate::osxnav_window::OsxNavWindow;

fn main() {
    let mut config = WindowConfig::default();
    config.set_initial_dimensions(0., 0., 0., 0.);
    config.set_styles(&[WindowStyle::Borderless]);

    App::new("xoc3.osxnav", OsxNavApp {
        window: Window::with(config, OsxNavWindow::default()),
        key_monitor: RwLock::new(None)
    }).run();
}

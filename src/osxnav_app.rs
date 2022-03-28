use std::sync::RwLock;
use cacao::macos::{App, AppDelegate, Event, EventMask, EventMonitor};
use cacao::macos::window::Window;
use cacao::notification_center::Dispatcher;
use core_graphics::base::CGFloat;
use crate::draw::draw_grid;
use crate::osxnav::{dispatch, Key};
use crate::osxnav_window::OsxNavWindow;

pub struct OsxNavApp {
    pub window: Window<OsxNavWindow>,
    pub key_monitor: RwLock<Option<EventMonitor>>
}

impl OsxNavApp {
    // Based off https://github.com/ryanmcgrath/cacao/blob/f558f8e24d6c4f869a4135bd230222455a435dcf/examples/calculator/main.rs
    fn start_monitoring(&self) {
        let mut lock = self.key_monitor.write().unwrap();
        *lock = Some(Event::local_monitor(EventMask::KeyDown, |evt| {
            let characters = evt.characters();
            match characters.as_ref() {
                "h" => dispatch(Key::H),
                "j" => dispatch(Key::J),
                "k" => dispatch(Key::K),
                "l" => dispatch(Key::L),
                _ => {}
            }
            None
        }));
    }
}
impl AppDelegate for OsxNavApp {
    fn did_finish_launching(&self) {
        App::activate();
        self.start_monitoring();
        self.window.show();
    }
}
impl Dispatcher for OsxNavApp {
    type Message = [(CGFloat, CGFloat); 2];

    fn on_ui_message(&self, xy_modifiers: Self::Message) {
        if let Some(delegate) = &self.window.delegate {
            let xy_min_modifiers = xy_modifiers[0];
            let xy_max_modifiers = xy_modifiers[1];

            // Just placeholders for now. Not sure how to actually get these values for a changed grid.
            let old_xmin = 0.;
            let old_ymin = 0.;
            let old_xmax = delegate.config.source.0;
            let old_ymax = delegate.config.source.1;

            // Not sure if this is the best way, but it works. (should actually work when we can get last position of grid)
            let xmin = old_xmin + xy_min_modifiers.0 * (old_xmax - old_xmin)/2.;
            let ymin = old_ymin + xy_min_modifiers.1 * (old_ymax - old_ymin)/2.;
            let xmax = old_xmax - xy_max_modifiers.0 * (old_xmax - old_xmin)/2.;
            let ymax = old_ymax - xy_max_modifiers.1 * (old_ymax - old_ymin)/2.;

            let image = draw_grid(delegate.config, xmin, ymin, xmax, ymax);
            delegate.image_view.set_image(&image);
        }
    }
}

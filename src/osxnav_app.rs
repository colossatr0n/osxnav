use std::sync::RwLock;
use cacao::image::DrawConfig;
use cacao::objc::{msg_send, sel, sel_impl, class};
use cacao::macos::{App, AppDelegate, Event, EventMask, EventMonitor};
use cacao::macos::window::Window;
use cacao::notification_center::Dispatcher;
use cacao::objc::runtime::Object;
use cacao::utils::CGSize;
use core_graphics::base::CGFloat;
use core_graphics::display::{CGPoint, CGRect};
use core_graphics::image::CGImageRef;
use core_graphics::sys::CGImage;
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

            let grid_frame: CGRect = delegate.image_view.objc.get(|obj| unsafe {
                return msg_send![obj, frame];
            });

            let old_xmin = grid_frame.origin.x;
            let old_ymin = grid_frame.origin.y;
            let old_xmax = old_xmin + grid_frame.size.width;
            let old_ymax = old_ymin + grid_frame.size.height;

            let xmin = old_xmin + xy_min_modifiers.0 * (old_xmax - old_xmin)/2.;
            let ymin = old_ymin + xy_min_modifiers.1 * (old_ymax - old_ymin)/2.;
            let xmax = old_xmax - xy_max_modifiers.0 * (old_xmax - old_xmin)/2.;
            let ymax = old_ymax - xy_max_modifiers.1 * (old_ymax - old_ymin)/2.;

            let grid_length = xmax - xmin;
            let grid_height = ymax - ymin;

            delegate.image_view.objc.get(|obj| unsafe {
                let _: ()= msg_send![obj, setFrameSize:CGSize::new(grid_length, grid_height)];
                let _: () = msg_send![obj, setFrameOrigin:CGPoint::new(xmin, ymin)];
            });

            let config = DrawConfig {
                source: (grid_length, grid_height),
                target: (grid_length, grid_height),
                resize: cacao::image::ResizeBehavior::Stretch,
            };

            let image = draw_grid(config, 0., 0., grid_length, grid_height);

            delegate.image_view.set_image(&image);
        }
    }
}

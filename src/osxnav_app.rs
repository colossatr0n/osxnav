use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;
use cacao::foundation::NSUInteger;
use cacao::image::{DrawConfig, ImageView};
use cacao::objc::{msg_send, sel, sel_impl, class};
use cacao::macos::{App, AppDelegate, Event, EventMask, EventMonitor};
use cacao::macos::window::Window;
use cacao::notification_center::Dispatcher;
use cacao::objc::runtime::Object;
use cacao::utils::CGSize;
use core_graphics::base::CGFloat;
use core_graphics::display::{CGPoint, CGRect};
use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
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
            // We'll use modifiers at some point
            let modifier_flags: NSUInteger = unsafe {
                msg_send![&*evt.0, modifierFlags]
            };

            let keycode: NSUInteger = unsafe {
                msg_send![&*evt.0, keyCode]
            };

            match keycode {
                Key::H | Key::J | Key::K | Key::L | Key::RETURN | Key::ESC => dispatch(keycode),
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
    type Message = NSUInteger;

    fn on_ui_message(&self, message: Self::Message) {
        if let Some(delegate) = &self.window.delegate {
            // There's probably a better way of doing this than replicating the same switch statement conditions in osxnav.rs
            match message {
                // Need to pass some kind of data that indicates if x/y/min/max need to be altered
                // Just using this list of tuples for now.
                Key::H => {
                    reposition_grid(&delegate.image_view, [(0., 0.), (1., 0.)]);
                },
                Key::J => {
                    reposition_grid(&delegate.image_view, [(0., 1.), (0., 0.)]);
                },
                Key::K => {
                    reposition_grid(&delegate.image_view, [(0., 0.), (0., 1.)]);
                },
                Key::L => {
                    reposition_grid(&delegate.image_view, [(1., 0.), (0., 0.)]);
                },
                Key::RETURN => {
                    let grid_frame: CGRect = delegate.image_view.objc.get(|obj| unsafe {
                        return msg_send![obj, frame];
                    });

                    let xmin = grid_frame.origin.x;
                    let ymin = grid_frame.origin.y;
                    let width = grid_frame.size.width;
                    let height = grid_frame.size.height;

                    let click_point = (xmin + width/2., ymin + height/2.);

                    // This isn't an actual "double click." Some applications need this for the
                    // click to go through such as web browsers (Chrome). Might be a better way to do this.
                    click(click_point.0, click_point.1);
                    click(click_point.0, click_point.1);
                    
                    // Sleep to allow click event to occur before exiting. 
                    sleep(Duration::from_millis(10));
                    std::process::exit(0);
                },
                _ => { println!("{}", message)
                }
            }
        }

    }
}

fn click(x: CGFloat, y: CGFloat) {
    let mouseDown = CGEvent::new_mouse_event(
        CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap(),
        CGEventType::LeftMouseDown,
        CGPoint::new(x, y),
        CGMouseButton::Left
    );

    let mouseUp = CGEvent::new_mouse_event(
        CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap(),
        CGEventType::LeftMouseUp,
        CGPoint::new(x, y),
        CGMouseButton::Left
    );

    mouseDown.unwrap().post(CGEventTapLocation::HID);
    sleep(Duration::from_millis(10));
    mouseUp.unwrap().post(CGEventTapLocation::HID);
}


fn reposition_grid(image_view: &ImageView, xy_modifiers: [(CGFloat, CGFloat); 2]) {
    let xy_min_modifiers = xy_modifiers[0];
    let xy_max_modifiers = xy_modifiers[1];

    let grid_frame: CGRect = image_view.objc.get(|obj| unsafe {
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

    image_view.objc.get(|obj| unsafe {
        let _: ()= msg_send![obj, setFrameSize:CGSize::new(grid_length, grid_height)];
        let _: () = msg_send![obj, setFrameOrigin:CGPoint::new(xmin, ymin)];
    });

    let config = DrawConfig {
        source: (grid_length, grid_height),
        target: (grid_length, grid_height),
        resize: cacao::image::ResizeBehavior::Stretch,
    };

    let image = draw_grid(config, 0., 0., grid_length, grid_height);

    image_view.set_image(&image);
}

use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;
use cacao::foundation::NSUInteger;
use cacao::image::{DrawConfig, ImageView};
use cacao::objc::{msg_send, sel, sel_impl, class};
use cacao::macos::{App, AppDelegate, Event, EventMask, EventModifierFlag, EventMonitor};
use cacao::macos::window::Window;
use cacao::notification_center::Dispatcher;
use cacao::objc::runtime::Object;
use cacao::utils::CGSize;
use core_graphics::base::CGFloat;
use core_graphics::display::{CGPoint, CGRect};
use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGEventType, CGMouseButton};
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
            let modifier_flags: CGEventFlags = unsafe {
                msg_send![&*evt.0, modifierFlags]
            };

            let keycode: NSUInteger = unsafe {
                msg_send![&*evt.0, keyCode]
            };

            match keycode {
                Key::H | Key::J | Key::K | Key::L | Key::RETURN | Key::ESC => dispatch(keycode, modifier_flags),
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
    type Message = (NSUInteger, CGEventFlags);

    fn on_ui_message(&self, message: Self::Message) {
        if let Some(delegate) = &self.window.delegate {
            let keycode = message.0;
            let modifier_flags: CGEventFlags  = message.1;

            let mut shift_effect = 0.;
            if modifier_flags.contains(CGEventFlags::CGEventFlagShift) {
                shift_effect = -1.;
            }

            match keycode {
                Key::H => {
                    reposition_grid(&delegate.image_view, [(shift_effect, 0.), (1., 0.)]);
                },
                Key::J => {
                    reposition_grid(&delegate.image_view, [(0., 1.), (0., shift_effect)]);
                },
                Key::K => {
                    reposition_grid(&delegate.image_view, [(0., shift_effect), (0., 1.)]);
                },
                Key::L => {
                    reposition_grid(&delegate.image_view, [(1., 0.), (shift_effect, 0.)]);
                },
                Key::RETURN => {
                    send_click(&delegate.image_view)
                },
                _ => { println!("{}", keycode) }
            }
        }

    }
}

fn send_click(image_view: &ImageView) {
    let grid_frame: CGRect = image_view.objc.get(|obj| unsafe {
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

    // Sleep to allow click event to occur before exiting. Probably a better way to do this.
    sleep(Duration::from_millis(10));
    std::process::exit(0);
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

    let mut xmin = old_xmin + xy_min_modifiers.0 * (old_xmax - old_xmin)/2.;
    let mut ymin = old_ymin + xy_min_modifiers.1 * (old_ymax - old_ymin)/2.;
    let mut xmax = old_xmax - xy_max_modifiers.0 * (old_xmax - old_xmin)/2.;
    let mut ymax = old_ymax - xy_max_modifiers.1 * (old_ymax - old_ymin)/2.;

    let screens: *mut Object = unsafe { msg_send![class!(NSScreen), screens] };
    let screen: *mut Object = unsafe { msg_send![screens, objectAtIndex:0] };
    let frame: CGRect = unsafe { msg_send![screen, frame] };
    let screen_w = frame.size.width;
    let screen_h = frame.size.height;

    // If movement is outside of screen, move OOB side to edge of screen and retain previous grid
    // size
    if xmin < 0.0 {
        xmin = 0.0;
        xmax = old_xmax - (old_xmin);
    }
    if ymin < 0.0 {
        ymin = 0.0;
        ymax = old_ymax - old_ymin;
    }
    if xmax > screen_w {
        xmax = screen_w;
        xmin = screen_w - (old_xmax - old_xmin);
    }
    if ymax > screen_h {
        ymax = screen_h ;
        ymin = screen_h - (old_ymax - old_ymin);
    }

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

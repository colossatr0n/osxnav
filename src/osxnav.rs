use std::sync::{Arc, RwLock};
use cacao::foundation::NSUInteger;
use cacao::lazy_static::lazy_static;
use cacao::macos::App;
use core_graphics::base::CGFloat;
use core_graphics::display::CGPoint;
use core_graphics::event::{CGEvent, CGEventType, CGMouseButton, CGKeyCode, KeyCode, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use crate::OsxNavApp;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Key;
impl Key {
    // Find keycodes here for newer versions of MacOS: /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h
    pub const H: NSUInteger = 0x04;
    pub const J: NSUInteger = 0x26;
    pub const K: NSUInteger = 0x28;
    pub const L: NSUInteger = 0x25;
    pub const RETURN: NSUInteger = 0x24;
}

// Based off https://github.com/ryanmcgrath/cacao/blob/f558f8e24d6c4f869a4135bd230222455a435dcf/examples/calculator/calculator.rs

lazy_static! {
    pub static ref OSXNAV: OsxNav = OsxNav::new();
}

// Asynchronously calls back through to the top of the application on the main thread.
pub fn dispatch(key: NSUInteger) {
    println!("Dispatching UI message: {:?}", key);
    OSXNAV.run(key)
}

pub struct OsxNav(Arc<RwLock<Vec<String>>>);

impl OsxNav {
    pub fn new() -> Self {
        OsxNav(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn run(&self, message: NSUInteger) {
        match message {
            // Need to pass some kind of data that indicates if x/y/min/max need to be altered
            // Just using this list of tuples for now.
            Key::H => {
                App::<OsxNavApp, NSUInteger>::dispatch_main(message);
            },
            Key::J => {
                App::<OsxNavApp, NSUInteger>::dispatch_main(message);
            },
            Key::K => {
                App::<OsxNavApp, NSUInteger>::dispatch_main(message);
            },
            Key::L => {
                App::<OsxNavApp, NSUInteger>::dispatch_main(message);
            },
            Key::RETURN => {
                App::<OsxNavApp, NSUInteger>::dispatch_main(message);
            },
            _ => {}
        }
    }
}

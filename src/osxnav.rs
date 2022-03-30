use std::sync::{Arc, RwLock};
use cacao::foundation::NSUInteger;
use cacao::lazy_static::lazy_static;
use cacao::macos::App;
use core_graphics::base::CGFloat;
use core_graphics::display::CGPoint;
use core_graphics::event::{CGEvent, CGEventType, CGMouseButton, CGKeyCode, KeyCode, CGEventTapLocation, CGEventFlags};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use crate::OsxNavApp;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Key;
impl Key {
    // Keycodes local ref: /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h
    // Keycodes online ref: https://eastmanreference.com/complete-list-of-applescript-key-codes
    pub const H: NSUInteger = 0x04;
    pub const J: NSUInteger = 0x26;
    pub const K: NSUInteger = 0x28;
    pub const L: NSUInteger = 0x25;
    pub const RETURN: NSUInteger = 0x24;
    pub const ESC: NSUInteger = 0x35;
}

// Based off https://github.com/ryanmcgrath/cacao/blob/f558f8e24d6c4f869a4135bd230222455a435dcf/examples/calculator/calculator.rs

lazy_static! {
    pub static ref OSXNAV: OsxNav = OsxNav::new();
}

// Asynchronously calls back through to the top of the application on the main thread.
pub fn dispatch(key: NSUInteger, modifier_flags: CGEventFlags) {
    println!("Dispatching UI message: {:?}", key);
    OSXNAV.run(key, modifier_flags)
}

pub struct OsxNav(Arc<RwLock<Vec<String>>>);

impl OsxNav {
    pub fn new() -> Self {
        OsxNav(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn run(&self, keycode: NSUInteger, modifier_flags: CGEventFlags) {
        match keycode {
            Key::H | Key::J | Key::K | Key::L | Key::RETURN => {
                App::<OsxNavApp, (NSUInteger, CGEventFlags)>::dispatch_main((keycode, modifier_flags));
            },
            Key::ESC => {
                // Possibly a bad way to end the program. Works for now though.
                std::process::exit(0);
            },
            _ => {}
        }
    }
}

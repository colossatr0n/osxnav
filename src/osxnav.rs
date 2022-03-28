use std::sync::{Arc, RwLock};
use cacao::lazy_static::lazy_static;
use cacao::macos::App;
use core_graphics::base::CGFloat;
use crate::OsxNavApp;

// Based off https://github.com/ryanmcgrath/cacao/blob/f558f8e24d6c4f869a4135bd230222455a435dcf/examples/calculator/calculator.rs

#[derive(Clone, Debug)]
pub enum Key {
    H,
    J,
    K,
    L,
}

lazy_static! {
    pub static ref OSXNAV: OsxNav = OsxNav::new();
}

// Asynchronously calls back through to the top of the application on the main thread.
pub fn dispatch(key: Key) {
    println!("Dispatching UI message: {:?}", key);
    OSXNAV.run(key)
}

pub struct OsxNav(Arc<RwLock<Vec<String>>>);

impl OsxNav {
    pub fn new() -> Self {
        OsxNav(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn run(&self, message: Key) {
        match message {
            // Need to pass some kind of data that indicates if x/y/min/max need to be altered
            // Just using this list of tuples for now.
            Key::H => {
                App::<OsxNavApp, [(CGFloat, CGFloat); 2]>::dispatch_main([(0., 0.), (1., 0.)]);
            },
            Key::J => {
                App::<OsxNavApp, [(CGFloat, CGFloat); 2]>::dispatch_main([(0., 1.), (0., 0.)]);
            },
            Key::K => {
                App::<OsxNavApp, [(CGFloat, CGFloat); 2]>::dispatch_main([(0., 0.), (0., 1.)]);
            },
            Key::L => {
                App::<OsxNavApp, [(CGFloat, CGFloat); 2]>::dispatch_main([(1., 0.), (0., 0.)]);
            }
        }
    }
}

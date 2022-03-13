//! This example showcases setting up a basic application and window delegate.
//! Window Delegate's give you lifecycle methods that you can respond to.

use core_graphics::geometry::{CGRect, CGPoint, CGSize};
use cacao::layout::Layout;
use cacao::macos::{App, AppDelegate};
use cacao::macos::window::{Window, WindowStyle, WindowConfig, WindowDelegate};
use cacao::color::{Color};
use cacao::view::View;
use cacao::objc::{msg_send, sel, sel_impl, class};

use std::os::raw::{c_char, c_int, c_uint, c_void};
use cacao::objc::runtime::{Object, Class, objc_copyClassList};
use cacao::foundation::{nil, YES, NO, NSString, NSInteger, NSUInteger};

// Reference can be found here
// https://github.com/phracker/MacOSX-SDKs/blob/041600eda65c6a668f66cb7d56b7d1da3e8bcc93/MacOSX10.10.sdk/System/Library/Frameworks/AppKit.framework/Versions/C/Headers/NSWindow.h
// const NSWindowStyleMaskFullSizeContentView: i32 = 1 << 15;

struct BasicApp {
    window: Window<MyWindow>
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::activate();
        self.window.show();
    }
}

#[derive(Default)]
struct MyWindow {
    content: View,
}

impl WindowDelegate for MyWindow {
    const NAME: &'static str = "MyWindow";

    fn did_load(&mut self, window: Window) {
        self.content.set_background_color(Color::SystemBlue);
        self.content.set_needs_display(true);
        self.content.set_translates_autoresizing_mask_into_constraints(true);
        self.content.set_frame(CGRect::new(&CGPoint::new(-100., 100.), &CGSize::new(100., 100.)));
        window.set_content_view(&self.content);

        // window.set_minimum_content_size(50,50);
        // window.set_maximum_content_size(50,50);
        // window.set_content_size(50,50);
        // window.set_minimum_size(50,50);

        window.set_title_visibility(cacao::macos::window::TitleVisibility::Hidden);
        window.set_background_color(Color::Clear);
        window.set_titlebar_appears_transparent(true);
        window.set_excluded_from_windows_menu(true);
        window.set_shows_toolbar_button(false);
        window.set_titlebar_appears_transparent(true);

        // debug
        window.set_movable_by_background(true);

        // Needed to move the window according to the screen.
        unsafe {
            let _: () = msg_send![&*window.objc, setHidesOnDeactivate:NO];
            let _: () = msg_send![&*window.objc, setLevel:1 << 30];
            let _: () = msg_send![&*window.objc, setAnimationBehavior:nil];
        }
    }

    fn will_close(&self) {
        println!("Closing now!");
    }

    fn will_enter_full_screen(&self) {
        println!("FULL SCREEN now!");
    }

    fn will_move(&self) {
        println!("Will move...");
    }

    fn did_move(&self) {
        println!("Did move...");
    }

    fn will_resize(&self, width: f64, height: f64) -> (f64, f64) {
        println!("Resizing to: {} {}", width, height);
        (width, height)
    }
}

fn main() {
    let mut config = WindowConfig::default();
    config.set_initial_dimensions(100., 1100., 100., 100.);

    config.set_styles(&[WindowStyle::Borderless]);

    App::new("com.test.window-delegate", BasicApp {
        window: Window::with(config, MyWindow::default())
    }).run();
}

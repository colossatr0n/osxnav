use core_graphics::geometry::{CGRect, CGPoint, CGSize};
use core_graphics::context::{CGContext, CGContextRef};

use cacao::layout::Layout;
use cacao::macos::{App, AppDelegate};
use cacao::macos::window::{Window, WindowStyle, WindowConfig, WindowDelegate};
use cacao::color::{Color};
use cacao::view::View;
use cacao::objc::{msg_send, sel, sel_impl, class};
use cacao::image::{ImageView, Image, DrawConfig};


use std::os::raw::{c_char, c_int, c_uint, c_void};
use cacao::objc::runtime::{Object, Class, objc_copyClassList};
use cacao::foundation::{NSArray, nil, YES, NO, NSString, NSInteger, NSUInteger};


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

fn draw_line(context: &CGContextRef, x1: f64, y1: f64, x2: f64, y2: f64) {
    context.move_to_point(x1, y1);
    context.add_line_to_point(x2, y2);
}

impl WindowDelegate for MyWindow {
    const NAME: &'static str = "MyWindow";

    fn did_load(&mut self, window: Window) {
        let screens: *mut Object = unsafe { msg_send![class!(NSScreen), screens] };
        let screen: *mut Object = unsafe { msg_send![screens, objectAtIndex:0] };
        let frame: CGRect = unsafe { msg_send![screen, frame] };
        let screen_w = frame.size.width;
        let screen_h = frame.size.height;
        window.set_content_size(screen_w, screen_h);

        let image_view = ImageView::new();
        let config = DrawConfig {
            source: (screen_w, screen_h),
            target: (screen_w, screen_h),
            resize: cacao::image::ResizeBehavior::Stretch,
        };

        let image = Image::draw(config, move |_cg_rect, context| {
            let xmin = 0.;
            let ymin = 0.;
            let xmax = screen_w;
            let ymax = screen_h;

            context.move_to_point(xmin, ymin);
            context.add_line_to_point(xmin, ymax);
            context.add_line_to_point(xmax, ymax);
            context.add_line_to_point(xmax, ymin);
            context.add_line_to_point(xmin, ymin);
            context.add_line_to_point(xmin, ymax);

            context.move_to_point(xmin, (ymax + ymin) / 2.);
            context.add_line_to_point(xmax, (ymax + ymin) / 2.);

            context.move_to_point((xmax + xmin) / 2., ymin);
            context.add_line_to_point((xmax + xmin) / 2., ymax);

            // draw_line(context, 0.,   0.,   0.,   ymax);
            // draw_line(context, 0.,   ymax, xmax, ymax);
            // draw_line(context, xmax, ymax, xmax, 0.);
            // draw_line(context, xmax, 0.,   0.,   0.);

            context.set_rgb_stroke_color(1., 0., 0., 1.);
            context.stroke_path();
            true
        });

        image_view.set_image(&image);
        self.content.add_subview(&image_view);
        self.content.set_needs_display(true);
        self.content.set_translates_autoresizing_mask_into_constraints(true);
        // self.content.set_frame(CGRect::new(&CGPoint::new(-100., 100.), &CGSize::new(100., 100.)));
        window.set_content_view(&self.content);

        // window.set_minimum_content_size(50,50);
        // window.set_maximum_content_size(50,50);
        // window.set_content_size(50,50);
        // window.set_minimum_size(50,50);

        window.set_title_visibility(cacao::macos::window::TitleVisibility::Hidden);
        window.set_background_color(Color::Clear);
        // window.set_background_color(Color::SystemBlue);
        window.set_titlebar_appears_transparent(true);
        window.set_excluded_from_windows_menu(true);
        window.set_shows_toolbar_button(false);
        window.set_titlebar_appears_transparent(true);

        // Needed to move the window according to the screen.

        let _: () = unsafe { msg_send![&*window.objc, setHidesOnDeactivate:NO] };
        let _: () = unsafe { msg_send![&*window.objc, setLevel:1 << 30] };
        let _: () = unsafe { msg_send![&*window.objc, setAnimationBehavior:nil] };

        // debug
        // window.set_movable_by_background(true);
    }

    fn will_resize(&self, width: f64, height: f64) -> (f64, f64) {
        println!("Resizing to: {} {}", width, height);
        (width, height)
    }
}

fn main() {
    let mut config = WindowConfig::default();
    config.set_initial_dimensions(0., 0., 0., 0.);
    config.set_styles(&[WindowStyle::Borderless]);

    App::new("xoc3.osxnav", BasicApp {
        window: Window::with(config, MyWindow::default())
    }).run();
}

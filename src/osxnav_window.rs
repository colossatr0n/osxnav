use cacao::color::Color;
use cacao::image::{DrawConfig, ImageView};
use cacao::layout::Layout;
use cacao::macos::window::{Window, WindowDelegate};
use cacao::objc::{msg_send, sel, sel_impl, class};
use cacao::objc::runtime::Object;
use cacao::view::View;
use core_graphics::display::CGRect;
use cacao::foundation::{NSArray, nil, YES, NO, NSString, NSInteger, NSUInteger};
use crate::draw::draw_grid;


pub struct OsxNavWindow {
   pub content: View,
   pub image_view: ImageView,
   pub config: DrawConfig,
}

impl Default for OsxNavWindow {
    fn default() -> OsxNavWindow {
        OsxNavWindow {
            content: View::default(),
            image_view: ImageView::default(),
            // Just initializing it to something.
            config: DrawConfig {
                source: (1., 1.),
                target: (1., 1.),
                resize: cacao::image::ResizeBehavior::Stretch,
            },
        }
    }
}

impl WindowDelegate for OsxNavWindow {
    const NAME: &'static str = "OsxNavWindow";

    fn did_load(&mut self, window: Window) {
        let screens: *mut Object = unsafe { msg_send![class!(NSScreen), screens] };
        let screen: *mut Object = unsafe { msg_send![screens, objectAtIndex:0] };
        let frame: CGRect = unsafe { msg_send![screen, frame] };
        let screen_w = frame.size.width;
        let screen_h = frame.size.height;
        window.set_content_size(screen_w, screen_h);

        self.image_view = ImageView::new();
        self.config = DrawConfig {
            source: (screen_w, screen_h),
            target: (screen_w, screen_h),
            resize: cacao::image::ResizeBehavior::Stretch,
        };

        let image = draw_grid(self.config, 0., 0., self.config.source.0, self.config.source.1);
        self.image_view.set_image(&image);
        self.content.add_subview(&self.image_view);
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
        // let _: () = unsafe { msg_send![&*window.objc, canBecomeMainWindow:NO] };
        // let _: () = unsafe { msg_send![&*window.objc, canBecomeKeyWindow:NO] };

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

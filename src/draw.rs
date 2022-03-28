use cacao::image::{DrawConfig, Image};
use core_graphics::base::CGFloat;
use core_graphics::context::CGContextRef;

pub fn draw_line(context: &CGContextRef, x1: f64, y1: f64, x2: f64, y2: f64) {
    context.move_to_point(x1, y1);
    context.add_line_to_point(x2, y2);
}

pub fn draw_grid(
    config: DrawConfig,
    xmin: CGFloat,
    ymin: CGFloat,
    xmax: CGFloat,
    ymax: CGFloat,
) -> Image {
    let image = Image::draw(config, move |_cg_rect, context| {
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

        context.set_rgb_stroke_color(1., 0., 0., 1.);
        context.stroke_path();
        true
    });

    image
}

extern crate num_rational;
extern crate rusttype;

extern crate font_metrics;

use num_rational::Ratio;
use rusttype::{point, PositionedGlyph, Scale};

use font_metrics::read_font_from_filename;

fn main() {
    let font = read_font_from_filename("C:\\Windows\\Fonts\\constan.ttf");

    // The font size to use
    let size = 256.0;
    let scale = Scale { x: size, y: size };
    let origin = point(0.0, 0.0);


    let test_glyph: PositionedGlyph = font.glyph('g')
        .scaled(scale)
        .positioned(origin);

    let x_glyph: PositionedGlyph = font.glyph('x')
        .scaled(scale)
        .positioned(origin);

    let test_glyph_bb = test_glyph.pixel_bounding_box().unwrap();
    let x_glyph_bb = x_glyph.pixel_bounding_box().unwrap();

    println!("test {:?}", test_glyph_bb);
    println!("x {:?}", x_glyph_bb);

    let x_glyph_height = x_glyph_bb.max.y - x_glyph_bb.min.y;
    let y_direction_adjust = test_glyph_bb.min.y - x_glyph_bb.min.y;
    let test_glyph_width = test_glyph_bb.max.x - test_glyph_bb.min.x;

    let mut inked_pixels = 0;

    test_glyph.draw(|x, y, v| {
        let y: i32 = (y as i32) + y_direction_adjust;

        if y < 0 || y >= x_glyph_height {
            return;
        }

        if v > 0.5 {
            inked_pixels += 1;
        };
    });

    let area = x_glyph_height * test_glyph_width;
    let density = Ratio::new(inked_pixels, area);

    println!("density: {:?}", density);
}

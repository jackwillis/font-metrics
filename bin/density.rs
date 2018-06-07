extern crate rusttype;

extern crate font_metrics;

use rusttype::{point, PositionedGlyph, Scale};

use font_metrics::read_font_from_filename;

fn main() {
    let font = read_font_from_filename("C:\\Windows\\Fonts\\Ariblk.ttf");

    let font_size = 256.0;
    let scale = Scale {
        x: font_size,
        y: font_size,
    };
    let origin = point(0.0, 0.0);

    let get_glyph =
        |id: char| -> PositionedGlyph { font.glyph(id).scaled(scale).positioned(origin) };

    let test_alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
    let test_glyphs = test_alphabet.into_iter().map(get_glyph);

    let x_glyph = get_glyph('x');

    let densities: Vec<f64> = test_glyphs
        .map(|test_glyph| calculate_glyph_density(&x_glyph, &test_glyph))
        .collect();

    let densities_sum = densities.iter().sum::<f64>();
    let average_density = densities_sum / (densities.len() as f64);

    println!("average density: {:.3}", average_density);
}

fn calculate_glyph_density(x_glyph: &PositionedGlyph, test_glyph: &PositionedGlyph) -> f64 {
    let x_glyph_bb = x_glyph.pixel_bounding_box().unwrap();
    let test_glyph_bb = test_glyph.pixel_bounding_box().unwrap();

    let x_glyph_height = x_glyph_bb.max.y - x_glyph_bb.min.y;
    let y_direction_adjust = test_glyph_bb.min.y - x_glyph_bb.min.y;
    let test_glyph_width = test_glyph_bb.max.x - test_glyph_bb.min.x;

    let mut inked_pixels = 0;

    test_glyph.draw(|_x, y, v| {
        let y: i32 = (y as i32) + y_direction_adjust;

        let out_of_bounds = y < 0 || y >= x_glyph_height;

        if v > 0.5 && !out_of_bounds {
            inked_pixels += 1;
        };
    });

    let area = x_glyph_height * test_glyph_width;

    (inked_pixels as f64) / (area as f64)
}

extern crate clap;
extern crate rusttype;

extern crate font_metrics;

use rusttype::{Font, PositionedGlyph};

use font_metrics::read_font_from_filename;

struct DensityTestVariables<'a> {
    font: Font<'a>,
    resolution: f32,
}

fn main() {
    let test_vars = parse_args();
    let density = calculate_font_density(&test_vars);

    println!("density: {:.3}", density);
}

fn parse_args<'a>() -> DensityTestVariables<'a> {
    let matches = clap::App::new("density")
        .about(
            "Measures the density of TrueType fonts.\nCalculated from the amount inked between the baseline and x-height of lowercase Latin letters.",
        )
        .author("https://github.com/jackwillis/font-metrics/")
        .arg(
            clap::Arg::with_name("FILENAME")
                .help("The location of the TrueType font to measure (ex. C:\\Windows\\Fonts\\Constan.ttf)")
                .required(true)
                .index(1)
        )
        .get_matches();

    let filename = matches.value_of("FILENAME").unwrap();

    DensityTestVariables {
        font: read_font_from_filename(filename.to_owned()),
        resolution: 256.0,
    }
}

fn calculate_font_density(test_vars: &DensityTestVariables) -> f64 {
    let scale = rusttype::Scale {
        x: test_vars.resolution,
        y: test_vars.resolution,
    };
    let origin = rusttype::point(0.0, 0.0);

    let get_glyph =
        |id: char| -> PositionedGlyph { test_vars.font.glyph(id).scaled(scale).positioned(origin) };

    let test_alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
    let test_glyphs = test_alphabet.into_iter().map(get_glyph);

    let x_glyph = get_glyph('x');

    let densities: Vec<f64> = test_glyphs
        .map(|test_glyph| calculate_glyph_density(&x_glyph, &test_glyph))
        .collect();

    let densities_sum = densities.iter().sum::<f64>();
    densities_sum / (densities.len() as f64)
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

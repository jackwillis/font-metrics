extern crate clap;
extern crate font_metrics;
extern crate rusttype;
use font_metrics::read_font_from_filename;
use rusttype::{Font, PositionedGlyph};

struct DarknessTest<'a> {
    pub font: Font<'a>,

    /// Height (distance between descent and ascent) of test glyphs, in pixels.
    pub resolution: f32,
}

fn main() {
    let test_vars = parse_args();
    let density = measure_font_darkness(&test_vars);

    println!("darkness: {:.3}", density);
}

fn parse_args<'a>() -> DarknessTest<'a> {
    let matches = clap::App::new("fm-darkness")
        .about("Measures the darkness of TrueType fonts.")
        .author("https://github.com/jackwillis/font-metrics/")
        .arg(
            clap::Arg::with_name("FILENAME")
                .help("The location of the TrueType font to measure (ex. C:\\Windows\\Fonts\\Constan.ttf)")
                .required(true)
                .index(1)
        )
        .get_matches();

    let filename = matches.value_of("FILENAME").unwrap();

    DarknessTest {
        font: read_font_from_filename(filename.to_owned()),
        resolution: 256.0,
    }
}

fn measure_font_darkness(test: &DarknessTest) -> f64 {
    let scale = rusttype::Scale {
        x: test.resolution,
        y: test.resolution,
    };
    let origin = rusttype::point(0.0, 0.0);

    let get_glyph =
        |id: char| -> PositionedGlyph { test.font.glyph(id).scaled(scale).positioned(origin) };

    let x_glyph: PositionedGlyph = get_glyph('x');

    let test_alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
    let test_glyphs = test_alphabet.into_iter().map(get_glyph);

    let densities: Vec<f64> = test_glyphs
        .map(|test_glyph| get_glyph_density(&x_glyph, &test_glyph))
        .collect();

    let densities_sum: f64 = densities.iter().sum();
    densities_sum / (densities.len() as f64)
}

fn get_glyph_density(x_glyph: &PositionedGlyph, test_glyph: &PositionedGlyph) -> f64 {
    let x_glyph_box = x_glyph.pixel_bounding_box().unwrap();
    let test_glyph_box = test_glyph.pixel_bounding_box().unwrap();

    let x_glyph_height = x_glyph_box.max.y - x_glyph_box.min.y;
    let y_direction_adjust = test_glyph_box.min.y - x_glyph_box.min.y;
    let test_glyph_width = test_glyph_box.max.x - test_glyph_box.min.x;

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

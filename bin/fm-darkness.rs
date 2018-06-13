extern crate clap;
extern crate font_metrics;
extern crate rusttype;
use font_metrics::read_font_from_filename;
use rusttype::{Font, PositionedGlyph, Rect};

struct DarknessTest<'a> {
    pub font: Font<'a>,

    // Height (distance between descent and ascent) of test glyphs, in pixels.
    pub resolution: f32,
}

fn main() {
    let test = parse_args();
    let darkness = measure_font_darkness(&test);

    println!("darkness: {:.3}", darkness);
}

fn measure_font_darkness(test: &DarknessTest) -> f64 {
    let scale = rusttype::Scale {
        x: test.resolution,
        y: test.resolution,
    };
    let origin = rusttype::point(0.0, 0.0);

    let get_scaled_glyph = |id: char| -> PositionedGlyph {
        let glyph = test.font.glyph(id);
        glyph.scaled(scale).positioned(origin)
    };

    // The lowercase Latin alphabet should be a good enough test fixture...
    let test_alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
    let test_glyphs = test_alphabet.into_iter().map(get_scaled_glyph);

    // Only measure within the vertical bounds of the little 'x'.
    let x_glyph = get_scaled_glyph('x');
    let x_bounds = x_glyph.pixel_bounding_box().unwrap();

    let densities: Vec<f64> = test_glyphs
        .map(|glyph| measure_glyph_darkness(&glyph, &x_bounds))
        .collect();

    let densities_sum: f64 = densities.iter().sum();
    densities_sum / (densities.len() as f64)
}

fn measure_glyph_darkness(glyph: &PositionedGlyph, frame: &Rect<i32>) -> f64 {
    let glyph_bounds = glyph.pixel_bounding_box().unwrap();

    // Measure within the frame's vertical range...
    let height = frame.max.y - frame.min.y;
    let y_adjust = glyph_bounds.min.y - frame.min.y;

    // ...and within the test glyph's horizontal range
    let width = glyph_bounds.max.x - glyph_bounds.min.x;

    let mut inked_pixels = 0;

    // Iterate over every pixel in `glyph_bounds`...
    glyph.draw(|_x, y, value| {
        // ...But we really want to measure in `frame`.
        // Account for `frame` and `glyph_bounds` starting at different `.min.y`'s.
        let y: i32 = (y as i32) + y_adjust;

        // Return early if out of frame.
        if y < 0 || y >= height {
            return;
        }

        if value > 0.5 {
            inked_pixels += 1;
        };
    });

    let area = height * width;
    (inked_pixels as f64) / (area as f64)
}

fn parse_args<'a>() -> DarknessTest<'a> {
    let matches = clap::App::new("fm-darkness")
        .about("Measures the darkness of TrueType fonts.")
        .author("https://github.com/jackwillis/font-metrics/")
        .arg(
            clap::Arg::with_name("FILENAME")
                .help("The location of the TrueType font to measure (ex. C:\\Windows\\Fonts\\Constan.ttf).")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("resolution")
                .short("r")
                .long("resolution")
                .help("Height (distance between descent and ascent) of test glyphs, in pixels.")
                .takes_value(true)
                .default_value("256"),
        )
        .get_matches();

    let filename = matches.value_of("FILENAME").unwrap();
    let font = read_font_from_filename(filename.to_owned());

    let resolution = matches.value_of("resolution").unwrap()
        .parse::<i32>().unwrap() as f32;

    DarknessTest { font, resolution }
}

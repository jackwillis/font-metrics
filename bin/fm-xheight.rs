extern crate clap;
extern crate num_rational;
extern crate rusttype;

extern crate font_metrics;

use num_rational::Ratio;
use rusttype::{Font, Glyph};

use font_metrics::read_font_from_filename;

struct XHeightTest<'a> {
    font: Font<'a>,
}

fn main() {
    let test_variables = parse_args();
    let ratio = x_height_test(test_variables);

    println!(
        "x-height ratio: {} (~{:.3})",
        ratio,
        font_metrics::ratio_into_f32(ratio).unwrap()
    );
}

fn parse_args<'a>() -> XHeightTest<'a> {
    let matches = clap::App::new("xheight")
        .about("Calculates the x-height/cap height ratio of a TrueType font.")
        .author("https://github.com/jackwillis/font-metrics/")
        .arg(
            clap::Arg::with_name("FILENAME")
                .help("The location of the TrueType font to measure (ex. C:\\Windows\\Fonts\\Tahoma.ttf)")
                .required(true)
                .index(1)
        )
        .get_matches();

    let filename = matches.value_of("FILENAME").unwrap();
    let font = read_font_from_filename(filename.to_owned());

    XHeightTest { font }
}

fn x_height_test(XHeightTest { font }: XHeightTest) -> Ratio<i32> {
    // We measure the height of "vxz" and "HIT" to get
    // x-height and cap height, respectively.
    // These letters are used because they tend to stay
    // close to the actual x/cap heights, without overshooting.
    let x_height_glyphs = font.glyphs_for("vxz".chars());
    let cap_height_glyphs = font.glyphs_for("HIT".chars());

    let x_heights_sum = x_height_glyphs.map(glyph_height).sum::<i32>();
    let cap_heights_sum = cap_height_glyphs.map(glyph_height).sum::<i32>();

    if x_heights_sum == 0 {
        panic!("Glyphs H, I, and T all had zero height.");
    }

    Ratio::new(x_heights_sum, cap_heights_sum)
}

pub fn glyph_height(glyph: Glyph) -> i32 {
    let glyph = glyph.standalone();
    let bounding_box = glyph.get_data().unwrap().extents.unwrap();

    bounding_box.max.y - bounding_box.min.y
}

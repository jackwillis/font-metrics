extern crate clap;
extern crate num_rational;
extern crate rusttype;

use num_rational::Ratio;
use rusttype::{Font, FontCollection};

pub fn read_font_from_filename<'a>(filename: String) -> Font<'a> {
    let data = ::std::fs::read(&filename)
        .expect(format!("Couldn't open font file {}", &filename).as_str());

    let font_collection =
        FontCollection::from_bytes(data).expect("Could not parse font file as TrueType");

    font_collection
        .into_font()
        .expect("Font file contains multiple fonts")
}

pub fn ratio_into_f32(ratio: Ratio<i32>) -> Option<f32> {
    match ratio.denom() {
        0 => None,
        _ => Some(((*ratio.numer() as f64) / (*ratio.denom() as f64)) as f32),
    }
}

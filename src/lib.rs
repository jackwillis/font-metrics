extern crate num_rational;
extern crate rusttype;

use num_rational::Ratio;
use rusttype::{Font, FontCollection, Glyph, Rect};

pub fn x_height_ratio(font: &Font) -> Ratio<i32> {
    let x_height_glyphs = font.glyphs_for("vxz".chars());
    let cap_height_glyphs = font.glyphs_for("HIT".chars());

    let x_heights_sum = x_height_glyphs.map(glyph_height).sum::<i32>();
    let cap_heights_sum = cap_height_glyphs.map(glyph_height).sum::<i32>();

    Ratio::new(x_heights_sum, cap_heights_sum)
}

pub fn glyph_height(glyph: Glyph) -> i32 {
    let glyph = glyph.standalone();

    let extents: Rect<i32> = glyph.get_data().unwrap().extents.unwrap();

    extents.max.y - extents.min.y
}

pub fn read_font_from_filename(filename: &str) -> Font {
    let data = std::fs::read(filename).unwrap();
    let font_collection = FontCollection::from_bytes(data).unwrap();

    font_collection.into_font().unwrap()
}

pub fn ratio_into_f32(ratio: Ratio<i32>) -> Option<f32> {
    match ratio.denom() {
        0 => None,
        _ => Some(((*ratio.numer() as f64) / (*ratio.denom() as f64)) as f32),
    }
}

extern crate image;
extern crate rusttype;

extern crate font_metrics;

use image::{DynamicImage, Rgba};
use rusttype::{point, Glyph, PositionedGlyph, Scale};

use font_metrics::read_font_from_filename;

fn main() {
    let font = read_font_from_filename("C:\\Windows\\Fonts\\Tahoma.ttf");;

    // The font size to use
    let size = 256.0;
    let scale = Scale { x: size, y: size };
    let origin = point(0.0, 0.0);

    let glyph: Glyph = font.glyph('f');
    let glyph: PositionedGlyph = glyph.scaled(scale).positioned(origin);

    println!("f {:?}", font.glyph('f').scaled(scale).positioned(origin).pixel_bounding_box().unwrap());
    println!("x {:?}", font.glyph('x').scaled(scale).positioned(origin).pixel_bounding_box().unwrap());

    let bb = glyph.pixel_bounding_box().unwrap();
    let width = bb.max.x - bb.min.x;
    let height = bb.max.y - bb.min.y;
    let mut image = DynamicImage::new_rgba8(width as u32, height as u32).to_rgba();

    glyph.draw(|x, y, v| {
        let alpha: u8 = if v > 0.5 { 255 } else { 0 };
        let color = Rgba { data: [0, 0, 0, alpha] };

        //println!("{} {} {}", x, y, v);
        image.put_pixel(x, y, color);
    });

    font.glyph('x').scaled(scale).positioned(origin).draw(|x, y, v| {
        /*
        let alpha: u8 = if v > 0.5 { 255 } else { 0 };
        let color = Rgba { data: [0, 0, 0, alpha] };

        //println!("{} {} {}", x, y, v);
        image.put_pixel(x, y, color);
        */
        if v > 0.5 && x < (width as u32) && y < (height as u32) {
            image.put_pixel(x, y, Rgba { data: [200, 160, 220, 255] })
        }
    });

    // Save the image to a png file
    image.save("image_example.png").unwrap();
    println!("Generated: image_example.png");
}

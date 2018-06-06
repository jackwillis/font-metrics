extern crate clap;

extern crate num_rational;

extern crate font_metrics;

use font_metrics::ratio_into_f32;

fn main() {
    let app = clap::App::new("xheight")
        .about("Calculates the x-height/cap height ratio of a TrueType font.")
        .author("https://github.com/jackwillis/font-metrics/")
        .arg(
            clap::Arg::with_name("FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        );

    let matches = app.get_matches();
    let filename = matches.value_of("FILE").unwrap();

    let font = font_metrics::read_font_from_filename(filename);
    let ratio = font_metrics::x_height_ratio(&font);

    println!(
        "x-height ratio: {} (~{:.3})",
        ratio,
        ratio_into_f32(ratio).unwrap()
    );
}

extern crate clap;

extern crate font_metrics;

fn main() {
    let matches = clap::App::new("xheight")
        .about("Calculates the x-height/cap height ratio of a TrueType font.")
        .author("https://github.com/jackwillis/font-metrics/")
        .arg(
            clap::Arg::with_name("FILENAME")
                .help("The location of the TrueType font to measure")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("FILENAME").unwrap();
    let font = font_metrics::read_font_from_filename(filename);
    let ratio = font_metrics::x_height_ratio(&font);

    println!(
        "x-height ratio: {} (~{:.3})",
        ratio,
        font_metrics::ratio_into_f32(ratio).unwrap()
    );
}

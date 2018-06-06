extern crate clap;

extern crate lopdf;
extern crate num_rational;
extern crate pdf_extract;

extern crate font_metrics;

use num_rational::Ratio;

use font_metrics::ratio_into_f32;

fn main() {
    let matches = clap::App::new("cpp")
        .about("Calculates the characters per pica (cpp) of specially formatted PDF file.")
        .author("https://github.com/jackwillis/font-metrics/")
        .arg(
            clap::Arg::with_name("FILE")
                .help("The PDF file to analyze")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("picas")
                .short("w")
                .long("width")
                .help("Number of picas per line in the PDF file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    let picas_per_line = matches.value_of("picas").unwrap().parse::<i32>().unwrap();

    let document = lopdf::Document::load(filename).unwrap();
    let text = extract_text(document);

    let cpp = chars_per_line(text) / picas_per_line;

    println!(
        "characters per pica: {:?} (~{:.2})",
        cpp,
        ratio_into_f32(cpp).unwrap()
    );
}

fn chars_per_line(text: String) -> Ratio<i32> {
    let lines: Vec<&str> = text.trim().lines().collect();
    let lines_except_last = &lines[0..lines.len() - 1];

    let total_chars = lines_except_last
        .iter()
        .fold(0, |sum, line| sum + line.len());

    Ratio::new(total_chars as i32, lines.len() as i32)
}

fn extract_text(document: lopdf::Document) -> String {
    let mut buffer = String::new();
    {
        let mut output_device = pdf_extract::PlainTextOutput::new(&mut buffer);
        pdf_extract::output_doc(&document, &mut output_device);
    }
    buffer
}

extern crate clap;
extern crate lopdf;
extern crate num_rational;
extern crate pdf_extract;
extern crate tempdir;

extern crate font_metrics;

use std::fs::File;
use std::io::Write;

use num_rational::Ratio;

use font_metrics::ratio_into_f32;

fn main() {
    let matches = clap::App::new("cpp")
        .about("Calculates the characters per pica (cpp) of specially formatted PDF file.")
        .author("https://github.com/jackwillis/font-metrics/")
        .arg(
            clap::Arg::with_name("font")
                .short("f")
                .long("font")
                .help("Name of font to test")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("picas")
                .short("w")
                .long("width")
                .help("Number of picas per line")
                .takes_value(true)
                .default_value("32"),
        )
        .get_matches();

    let font_name = matches.value_of("font").unwrap();
    let picas_per_line = matches.value_of("picas").unwrap().parse::<i32>().unwrap();

    let working_dir = tempdir::TempDir::new("cpp").unwrap();

    let latex_file_path = working_dir.path().join("cpp.tex");

    let mut latex_file = File::create(&latex_file_path).unwrap();
    let source = generate_latex_source(font_name, 12, 36);
    latex_file.write_all(source.as_bytes()).unwrap();

    let mut xelatex = std::process::Command::new("xelatex");
    let command = xelatex
        .current_dir(&working_dir)
        .arg("-quiet")
        .arg("-interaction=nonstopmode")
        .arg(latex_file_path.to_str().unwrap().to_owned());

    println!("{:?}", &command);

    command.status().unwrap();

    let pdf_file_path = working_dir.path().join("cpp.pdf");

    let document = lopdf::Document::load(pdf_file_path).unwrap();
    let text = extract_text(document);

    let cpp = chars_per_line(text) / picas_per_line;

    println!(
        "characters per pica: {:?} (~{:.2})",
        cpp,
        ratio_into_f32(cpp).unwrap()
    );

    working_dir.close().unwrap();
}

fn chars_per_line(text: String) -> Ratio<i32> {
    let lines: Vec<&str> = text.trim().lines().collect();
    let lines_except_last = &lines[0..lines.len() - 1];

    let total_chars = lines_except_last
        .iter()
        .fold(0, |sum, line| sum + line.len());

    Ratio::new(total_chars as i32, lines.len() as i32)
}

fn generate_latex_source(font_name: &str, font_size: i32, text_width: i32) -> String {
    format!(
        r"
\documentclass[{font_size}pt]{{article}}
\usepackage[textwidth = {text_width}pc]{{geometry}}
\usepackage{{fontspec, microtype, blindtext}}
\pagestyle{{empty}}
\setmainfont{{{font_name}}}[OpticalSize = 0]
\begin{{document}}
\noindent
\blindtext
\end{{document}}
    ",
        font_name = font_name,
        font_size = font_size,
        text_width = text_width
    )
}

fn extract_text(document: lopdf::Document) -> String {
    let mut buffer = String::new();
    {
        let mut output_device = pdf_extract::PlainTextOutput::new(&mut buffer);
        pdf_extract::output_doc(&document, &mut output_device);
    }
    buffer
}

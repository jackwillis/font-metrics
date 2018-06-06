extern crate clap;
extern crate lopdf;
extern crate num_rational;
extern crate pdf_extract;
extern crate tempdir;

extern crate font_metrics;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use num_rational::Ratio;
use tempdir::TempDir;

use font_metrics::ratio_into_f32;

struct CppTestVariables {
    font_name: String,
    font_size: i32,
    text_width: i32
}

fn main() {
    let matches = clap::App::new("cpp")
        .about("Measures the characters per pica (cpp) of TrueType system fonts on a standard page.")
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
            clap::Arg::with_name("size")
                .short("s")
                .long("size")
                .help("Font size in points")
                .takes_value(true)
                .default_value("12"),
        )
        .arg(
            clap::Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Width of the test page's printable area in picas")
                .takes_value(true)
                .default_value("32"),
        )
        .get_matches();

    let test_vars = CppTestVariables {
        font_name: matches.value_of("font").unwrap().to_owned(),
        font_size: matches.value_of("size").unwrap().parse::<i32>().unwrap(),
        text_width: matches.value_of("width").unwrap().parse::<i32>().unwrap()
    };

    let dir = TempDir::new("cpp").unwrap();

    let pdf_path = generate_pdf(dir.path(), &test_vars);
    let cpp = analyze_pdf(&pdf_path, &test_vars);

    println!(
        "characters per pica: {:?} (~{:.2})",
        cpp,
        ratio_into_f32(cpp).unwrap()
    );
}

fn generate_pdf(working_dir: &Path, vars: &CppTestVariables) -> PathBuf {
    let tex_path = working_dir.join("cpp.tex");
    let mut file = File::create(&tex_path).unwrap();
    let source = generate_latex_source(vars);
    file.write_all(source.as_bytes()).unwrap();

    let mut xelatex = std::process::Command::new("xelatex");
    let command = xelatex
        .current_dir(&working_dir)
        .arg("-quiet")
        .arg("-interaction=nonstopmode")
        .arg(tex_path.into_os_string().into_string().unwrap());

    println!("{:?}", &command);

    command.status().unwrap();

    PathBuf::from(working_dir.join("cpp.pdf"))
}

fn generate_latex_source(vars: &CppTestVariables) -> String {
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
        font_name = vars.font_name,
        font_size = vars.font_size,
        text_width = vars.text_width
    )
}

fn analyze_pdf(path: &Path, vars: &CppTestVariables) -> Ratio<i32> {
    let document = lopdf::Document::load(path).unwrap();
    let text = extract_text(document);

    chars_per_line(text) / vars.text_width
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

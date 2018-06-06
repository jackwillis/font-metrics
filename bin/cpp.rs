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
    font_directory: String,
    font_size: i32,
    text_width: i32,
    verbose: bool
}

fn main() {
    let matches = clap::App::new("cpp")
        .about(
            "Measures the characters per pica (cpp) of TrueType fonts on a standard test page.",
        )
        .author("https://github.com/jackwillis/font-metrics/")
        .arg(
            clap::Arg::with_name("FILENAME")
                .help("The location of the TrueType font to measure (ex. C:\\Windows\\Fonts\\Tahoma.ttf)")
                .required(true)
                .index(1)
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
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Prints extra debug messages")
        )
        .get_matches();

    let font_filename = matches.value_of("FILENAME").unwrap();
    let font_path = Path::new(font_filename);

    if !font_path.is_file() {
        panic!(format!("{:?} is not a file!", font_path));
    }

    let test_vars = CppTestVariables {
        font_name: font_path.file_stem().unwrap().to_str().unwrap().to_owned(),
        font_directory: {
            let dir = font_path.parent().unwrap();
            let dir_str = dir.to_str().unwrap().to_owned();

            // xelatex hates Windows-style paths
            dir_str.replace("\\", "/")
        },
        font_size: matches.value_of("size").unwrap().parse::<i32>().unwrap(),
        text_width: matches.value_of("width").unwrap().parse::<i32>().unwrap(),
        verbose: matches.is_present("verbose")
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
    let mut file = File::create(&tex_path).expect("Couldn't create temp file");
    let source = generate_latex_source(vars);

    if vars.verbose {
        println!("{}", source);
    }

    file.write_all(source.as_bytes())
        .expect("Couldn't write to temp file");

    let mut xelatex = std::process::Command::new("xelatex");
    let command = xelatex
        .current_dir(&working_dir)
        .arg("-quiet")
        .arg("-interaction=nonstopmode")
        .arg(tex_path.into_os_string().into_string().unwrap());

    if vars.verbose {
        println!("{:?}", &command);
    }

    let status = command.status().expect("xelatex could not be found");
    if !status.success() {
        panic!("xelatex did not exit successfully");
    }

    PathBuf::from(working_dir.join("cpp.pdf"))
}

fn generate_latex_source(vars: &CppTestVariables) -> String {
    format!(
        r"
\documentclass[{font_size}pt]{{article}}
\usepackage[textwidth = {text_width}pc]{{geometry}}
\usepackage{{fontspec, microtype, blindtext}}
\pagestyle{{empty}}
\setmainfont{{{font_name}}}[OpticalSize = 0, Path = {font_directory}/]
\begin{{document}}
\noindent
\blindtext
\end{{document}}
    ",
        font_name = vars.font_name,
        font_directory = vars.font_directory,
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
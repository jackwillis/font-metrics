#[macro_use]
extern crate clap;
extern crate lopdf;
extern crate num_rational;
extern crate pdf_extract;
extern crate tempdir;

extern crate font_metrics;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::Arg;
use num_rational::Ratio;
use tempdir::TempDir;

use font_metrics::ratio_into_f32;

struct FitTest {
    font_name: String,
    font_directory: String,
    font_size: i32,
    text_width: i32,
    language: String,
    sample_text: String,
    display_as_ratio: bool,
    verbose: bool,
}

impl FitTest {
    fn generate_latex(&self) -> String {
        format!(
            r"
\documentclass{{article}}
\usepackage{{fontspec, microtype, polyglossia}}
\pagestyle{{empty}}
\setmainfont{{{font_name}}}[OpticalSize = 0, Path = {font_directory}/]
\setdefaultlanguage{{{language}}}
\setlength{{\textwidth}}{{{text_width}pc}}
\begin{{document}}
\fontsize{{{font_size}pt}}{{{font_size}pt}}\selectfont
\noindent
{sample_text}
\end{{document}}
            ",
            font_name = self.font_name,
            font_directory = self.font_directory,
            font_size = self.font_size,
            text_width = self.text_width,
            language = self.language,
            sample_text = self.sample_text
        )
    }
}

fn main() {
    let test: FitTest = parse_args();

    let temp_dir = TempDir::new("cpp").expect("Couldn't create temporary directory");
    let pdf_path = generate_pdf(temp_dir.path(), &test);

    let fit: Ratio<i32> = analyze_pdf(&pdf_path, &test);

    if test.display_as_ratio {
        println!("{:?}", fit);
    } else {
        println!("{:.2}", ratio_into_f32(fit).unwrap());
    }

    temp_dir
        .close()
        .expect("Couldn't delete temporary directory");
}

fn generate_pdf(working_dir: &Path, test: &FitTest) -> PathBuf {
    // Generate test article
    let article_path: PathBuf = working_dir.join("fit.tex");
    let mut article_file = File::create(&article_path).expect("Couldn't create temp file");

    let article_code = test.generate_latex();

    if test.verbose {
        println!("{}", article_code);
    }

    article_file
        .write_all(article_code.as_bytes())
        .expect(&format!(
            "Couldn't write to temporary file {:?}",
            article_file
        ));

    // Compile test article to PDF
    let article_path: String = article_path.into_os_string().into_string().unwrap();

    let mut lualatex = std::process::Command::new("lualatex");
    let command = lualatex
        .current_dir(&working_dir)
        .arg("--interaction=nonstopmode")
        .arg(article_path);

    if test.verbose {
        println!("{:?}", command);
    } else {
        // Suppress debug output from lualatex
        command.stdout(std::process::Stdio::null());
    };

    let status = command.status().expect("lualatex could not be found");
    if !status.success() {
        panic!("lualatex did not exit successfully");
    }

    // Return file path of generated PDF
    PathBuf::from(working_dir.join("fit.pdf"))
}

fn analyze_pdf(path: &Path, test: &FitTest) -> Ratio<i32> {
    let document = lopdf::Document::load(path).unwrap();
    let text = extract_text(document);

    avg_chars_per_line(text) / test.text_width
}

fn extract_text(document: lopdf::Document) -> String {
    let mut buffer = String::new();
    {
        let mut output_device = pdf_extract::PlainTextOutput::new(&mut buffer);
        pdf_extract::output_doc(&document, &mut output_device);
    }
    buffer
}

fn avg_chars_per_line(text: String) -> Ratio<i32> {
    let lines: Vec<&str> = text.trim().lines().collect();

    // throw away the last line since it's usually not full
    // and would skew the results
    let lines_except_last: &[&str] = &lines[0..lines.len() - 1];

    let total_chars = lines_except_last
        .iter()
        .fold(0, |sum, line| sum + line.len());

    Ratio::new(total_chars as i32, lines.len() as i32)
}

fn parse_args() -> FitTest {
    let matches = cli().get_matches();

    let font_path = {
        let font_filename = matches.value_of("filename").unwrap();
        Path::new(font_filename)
    };

    if !font_path.is_file() {
        panic!(format!("{:?} is not a file!", font_path));
    }

    let font_name = {
        let stem = font_path.file_stem().unwrap();
        stem.to_str().unwrap().to_owned()
    };

    let font_directory = {
        let dir = font_path.parent().unwrap();
        let dir_str = dir.to_str().unwrap().to_owned();
        dir_str.replace("\\", "/") // fontspec hates Windows-style paths
    };

    let font_size = value_t!(matches, "size", i32).unwrap_or_else(|e| e.exit());
    let text_width = value_t!(matches, "width", i32).unwrap_or_else(|e| e.exit());
    let language = value_t!(matches, "language", String).unwrap_or_else(|e| e.exit());

    let sample_text = match matches.value_of("sample") {
        Some(_filename) => "yikes",
        None => match language.as_str() {
            "english" => include_str!("../resources/english.example.tex"),
            "russian" => include_str!("../resources/russian.example.tex")
            _ => "uh oh",
        },
    }.to_owned();

    let display_as_ratio = matches.is_present("ratio");
    let verbose = matches.is_present("verbose");

    FitTest {
        font_name,
        font_directory,
        font_size,
        text_width,
        language,
        sample_text,
        display_as_ratio,
        verbose,
    }
}

fn cli() -> clap::App<'static, 'static> {
    clap::App::new("fm-fit")
        .about(
            "Measures the fit in characters per pica (cpp) of TrueType fonts on a standard LuaLaTeX test page.",
        )
        .version(crate_version!())
        .arg(
            Arg::with_name("filename")
                .value_name("FONT.ttf")
                .help("The TrueType font to measure (ex. C:\\Windows\\Fonts\\Arial.ttf)")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("points")
                .help("Set font size")
                .takes_value(true)
                .default_value("12"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("picas")
                .help("Set line width of the test page")
                .takes_value(true)
                .default_value("32"),
        )
        .arg(
            Arg::with_name("sample")
                .short("S")
                .long("sample")
                .value_name("filename.tex")
                .help("File containing sample text for the test. Optional for languages \"english\" and \"russian\", which have default sample texts.")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("language")
                .short("l")
                .long("language")
                .value_name("name")
                .help("Language package for polyglossia to load.")
                .takes_value(true)
                .default_value("english")
        )
        .arg(
            Arg::with_name("ratio")
                .short("r")
                .long("ratio")
                .help("Display answer as ratio instead of floating-point number.")
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Prints extra debug messages")
        )
}

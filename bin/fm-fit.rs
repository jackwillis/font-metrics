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

use clap::{Arg, Error, ErrorKind};
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
    verbose: bool,
}

fn main() {
    let test: FitTest = parse_args();

    let temp_dir = TempDir::new("cpp").expect("Couldn't create temporary directory");

    let pdf_path = generate_pdf(temp_dir.path(), &test);
    let fit: Ratio<i32> = analyze_pdf(&pdf_path, &test);

    println!(
        "fit in characters per pica: {:?} (~{:.2})",
        fit,
        ratio_into_f32(fit).unwrap()
    );

    temp_dir.close().expect("Couldn't delete temporary directory");
}

fn generate_pdf(working_dir: &Path, test: &FitTest) -> PathBuf {
    let tex_path: PathBuf = working_dir.join("cpp.tex");
    let mut file = File::create(&tex_path).expect("Couldn't create temp file");
    let latex_source = generate_latex_source(test);

    if test.verbose {
        println!("{}", latex_source);
    }

    file.write_all(latex_source.as_bytes())
        .expect("Couldn't write to temp file");

    let tex_path: String = tex_path.into_os_string().into_string().unwrap();
    let mut lualatex = std::process::Command::new("lualatex");
    let command = lualatex
        .current_dir(&working_dir)
        .arg("--interaction=nonstopmode")
        .arg(tex_path);

    if test.verbose {
        println!("{:?}", command);
    }

    let status = command.status().expect("lualatex could not be found");
    if !status.success() {
        panic!("lualatex did not exit successfully");
    }

    PathBuf::from(working_dir.join("cpp.pdf"))
}

fn generate_latex_source(test: &FitTest) -> String {
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
        font_name = test.font_name,
        font_directory = test.font_directory,
        font_size = test.font_size,
        text_width = test.text_width,
        language = test.language,
        sample_text = test.sample_text
    )
}

fn analyze_pdf(path: &Path, vars: &FitTest) -> Ratio<i32> {
    let document = lopdf::Document::load(path).unwrap();
    let text = extract_text(document);

    avg_chars_per_line(text) / vars.text_width
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

    FitTest {
        font_name: {
            let stem = font_path.file_stem().unwrap();
            stem.to_str().unwrap().to_owned()
        },
        font_directory: {
            let dir = font_path.parent().unwrap();
            let dir_str = dir.to_str().unwrap().to_owned();
            dir_str.replace("\\", "/") // fontspec hates Windows-style paths
        },
        font_size: {
            let capture = matches.value_of("size").unwrap();
            capture.parse::<i32>().unwrap()
        },
        text_width: {
            let capture = matches.value_of("width").unwrap();
            capture.parse::<i32>().unwrap()
        },
        language: {
            match matches.value_of("language") {
                // Manually setting a language overrides the presets
                Some(language) => language,

                None => {
                    if matches.is_present("english") {
                        "english"
                    }
                    else if matches.is_present("russian") {
                        "russian"
                    }
                    else {
                        let msg = "Language not set! Use --language <lang>, or a preset: --english or --russian.";
                        Error::with_description(msg, ErrorKind::EmptyValue).exit();
                    }
                }
            }.to_owned()
        },
        sample_text: {
            match matches.value_of("sample") {
                Some(_filename) => {
                    "yikes"
                },
                None => {
                    include_str!("../resources/english.example.tex")
                }
            }.to_owned()
        },
        verbose: matches.is_present("verbose"),
    }
}

fn cli() -> clap::App<'static, 'static> {
    clap::App::new("fm-fit")
        .about(
            "Measures the fit in characters per pica (cpp) of TrueType fonts on a standard LuaLaTeX test page.",
        )
        .author("https://github.com/jackwillis/font-metrics/")
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
                .help("File containing sample text for the test.")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("language")
                .short("l")
                .long("language")
                .value_name("name")
                .help("Language package for polyglossia (https://github.com/reutenauer/polyglossia) to load.")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("english")
                .short("E")
                .long("english")
                .help("Set --language to \"english\" and use built-in English sample text.")
        ).
        arg(
            Arg::with_name("russian")
                .short("R")
                .long("russian")
                .help("Set --language to \"russian\" and use built-in Russian sample text.")
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

// "Помимо рабочего движения, в Швейцарии существует и социалистическое женское движение. Издается двухнедельный журнал «Застрельщица». Социалистки разделяются на группы, которые по своим организационным формам сходны совершенно с партийной социалистической организацией. Тяжелые экономические условия во время войны, промышленный кризис, который уже в течение нескольких лет надвигается, и не прекращающееся вздорожание жизни пробудили женщину из инертного состояния и толкнули ее в ряды профессиональных и политических организаций. В последних громадных, массовых демонстрациях и забастовках женщины-пролетарии играли роль фермента, который двигал и толкал на дальнейшую борьбу. Особенно в больших городах происходили грандиозные женские демонстрации против дороговизны, которые заканчивались порой нападением на чиновников и разгромом лавок.",

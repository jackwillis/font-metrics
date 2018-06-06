extern crate font_metrics;
extern crate lopdf;
extern crate num_rational;
extern crate pdf_extract;

use font_metrics::ratio_into_f32;
use num_rational::Ratio;

fn main() {
    let filename = "cpp.pdf";
    let picas_per_line = 32;

    let document = lopdf::Document::load(filename).unwrap();
    let text = extract_text(document);

    let cpp = chars_per_line(text) / picas_per_line;

    println!("characters per pica: {:?} (~{:.2})",
             cpp, ratio_into_f32(cpp).unwrap());
}

fn chars_per_line(text: String) -> Ratio<i32> {
    let lines: Vec<&str> = text.trim().lines().collect();
    let lines_except_last = &lines[0..lines.len() - 1];

    let total_chars = lines_except_last.iter().fold(0, |sum, line| sum + line.len());

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
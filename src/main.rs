use std::fs::File;
use std::io::Read;

use clap::Parser;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};


#[derive(Parser, Debug)]
#[command(author="Nick Keers", name="rscat", about="A simple clone of cat in Rust")]
struct Cli {
    /// Number the non-blank output lines, starting at 1.
    #[arg(short='b', verbatim_doc_comment)]
    number_blank: bool,

    /// Syntax-highlight the file contents
    #[arg(short='s', verbatim_doc_comment, default_value_t=true)]
    syntax_highlight: bool,

    file: String
}

fn read_file(file_path: String) -> String {
    let file = File::open(file_path).expect("file must exist");

    let mut contents = String::new();
    std::io::BufReader::new(file).read_to_string(&mut contents).expect("reading file failed");

    contents
}

fn main() {
    let cli = Cli::parse();

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_for_file(&cli.file).unwrap().unwrap_or_else(|| ps.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    let contents = read_file(cli.file);

    if cli.number_blank {
        let mut line_number = 1;
        for line in LinesWithEndings::from(&contents) {
            if line == "" {
                println!();
            } else {
                if cli.syntax_highlight {
                    let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                    print!("{} {}", line_number, escaped);
                } else {
                    print!("{} {}", line_number, line);
                }
                line_number += 1;
            }
        }
    } else {
        if cli.syntax_highlight {
            for line in LinesWithEndings::from(&contents) {
                if line == "" {
                    continue;
                }

                let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                print!("{}", escaped);


            }
        } else {
            print!("{}", contents);
        }
    }
}

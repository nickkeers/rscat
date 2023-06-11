mod file_parser;

use std::io;
use clap::Parser;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet};
use file_parser::file_parser::FileParser;


#[derive(Parser, Debug)]
#[command(author="Nick Keers", name="rscat", about="A simple clone of cat in Rust")]
struct Cli {
    /// Number the non-blank output lines, starting at 1.
    #[arg(short='b', verbatim_doc_comment)]
    number_blank: bool,

    /// Squeeze multiple adjacent empty lines, causing the output to be single spaced.
    #[arg(short='s', verbatim_doc_comment, default_value_t=false)]
    squeeze: bool,

    /// Show non-printing characters, as in the -v option of GNU cat.
    #[arg(short='v', verbatim_doc_comment)]
    show_non_printing: bool,

    /// Syntax highlight the output
    #[arg(short='S', verbatim_doc_comment)]
    syntax_highlight: bool,

    /// Specify the language for syntax-highlighting explicitly
    #[arg(short='l', verbatim_doc_comment)]
    language: Option<String>,

    /// Specify the theme to use from the 'syntect' crate
    /// Choices: base16-ocean.dark, base16-eighties.dark, base16-mocha.dark, base16-ocean.light
    #[arg(short='t', verbatim_doc_comment, default_value="base16-ocean.dark")]
    theme: String,

    files: Vec<String>
}

fn main() {
    let cli = Cli::parse();

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let mut theme_name = cli.theme;

    if !ts.themes.contains_key(&theme_name) {
        theme_name = "base16-ocean.dark".to_string();
    }

    let theme = ts.themes.get(&theme_name).unwrap_or_else(|| ts.themes.get("base16-ocean.dark").unwrap());

    let file_parser = FileParser::new(cli.number_blank, cli.squeeze,cli.syntax_highlight, cli.show_non_printing);

    // let mut stdout_writer = BufWriter::new(io::stdout());
    for file in cli.files {
        let contents = file_parser.read_contents(file.clone());

        if contents.is_err() {
            eprintln!("Error: {}", contents.unwrap_err());
            continue;
        }

        let syntax = ps.find_syntax_for_file(file).unwrap().unwrap_or_else(|| ps.find_syntax_plain_text());
        let mut highlighter = HighlightLines::new(syntax, &theme);

        // move highlighter into the print_contents function
        file_parser.print_contents(&mut io::stdout(), &mut highlighter, &ps, contents.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_pos_arg() {
        let args = vec!["rscat", "Cargo.toml"];
        let result = Cli::parse_from(args);
        assert!(result.files.contains(&"Cargo.toml".to_string()));
    }

    #[test]
    fn test_main_pos_arg_multiple() {
        let args = vec!["rscat", "Cargo.toml", "Cargo.lock"];
        let result = Cli::parse_from(args);
        assert!(result.files.contains(&"Cargo.toml".to_string()));
        assert!(result.files.contains(&"Cargo.lock".to_string()));
    }

    #[test]
    fn test_main_pos_arg_multiple_with_flags() {
        let args = vec!["rscat", "-b", "-s", "-v", "-S", "-l", "rust", "-t", "base16-ocean.dark", "Cargo.toml", "Cargo.lock"];
        let result = Cli::parse_from(args);
        assert!(result.files.contains(&"Cargo.toml".to_string()));
        assert!(result.files.contains(&"Cargo.lock".to_string()));
        assert_eq!(result.number_blank, true);
        assert_eq!(result.squeeze, true);
        assert_eq!(result.show_non_printing, true);
        assert_eq!(result.syntax_highlight, true);
        assert_eq!(result.language, Some("rust".to_string()));
        assert_eq!(result.theme, "base16-ocean.dark".to_string());
    }

    #[test]
    fn test_theme_defaults_to_base16_ocean_dark() {
        let args = vec!["rscat", "Cargo.toml"];
        let result = Cli::parse_from(args);
        assert!(result.files.contains(&"Cargo.toml".to_string()));
        assert_eq!(result.theme, "base16-ocean.dark".to_string());
    }
}
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

#[derive(Copy, Clone)]
pub struct FileParser {
    line_numbers: bool,
    squeeze: bool,
    show_non_printing: bool,
    syntax_highlight: bool,
}

impl FileParser {
    pub fn new(number_lines: bool, squeeze_lines: bool, highlight_lines: bool, show_non_printing_chars: bool) -> FileParser {
        return FileParser{
            line_numbers: number_lines,
            squeeze: squeeze_lines,
            show_non_printing: show_non_printing_chars,
            syntax_highlight: highlight_lines,
        }
    }

    pub fn read_contents(self, file: String) -> Result<String, io::Error> {
        let opened = File::open(file)?;

        let mut contents = String::new();

        match io::BufReader::new(opened).read_to_string(&mut contents) {
            Ok(_size) => Ok(contents),
            Err(e) => Err(e),
        }
    }

    fn maybe_print_line_with_line_numbers<W: Write>(self, writer: &mut W, line: &str, num: &usize) {
        if self.line_numbers {
            write!(writer, "{} {}", num, line).expect("should be able to write to writer");
        } else {
            write!(writer, "{}", line).expect("should be able to write to writer");
        }
    }

    fn replace_non_printing_characters(self, line: &str) -> String {
        let mut new_line = String::new();

        if self.show_non_printing {
            for c in line.chars() {
                match c {
                    '\x0b' => new_line.push_str("^K"),
                    '\x0c' => new_line.push_str("^L"),
                    '\x0e' => new_line.push_str("^N"),
                    '\x0f' => new_line.push_str("^O"),
                    '\x1b' => new_line.push_str("^E"),
                    '\x7f' => new_line.push_str("^?"),
                    _ => new_line.push(c),
                }
            }
            return new_line;
        }

        return line.to_string();
    }

    pub(crate) fn print_contents<W: Write>(self, writer: &mut W, highlighter: &mut HighlightLines, ps: &SyntaxSet, contents: String) {
        let mut consecutive_blank_lines = 0;
        let mut num = 1;

        for line in LinesWithEndings::from(&contents) {
            // if the line is just whitespace with a newline at the end
            if line.trim().is_empty() {
                if self.squeeze.to_owned() {
                    consecutive_blank_lines += 1;
                    continue;
                }
            }

            let line = self.replace_non_printing_characters(line);

            // if squeeze is enabled we only want to print only one blank line
            if consecutive_blank_lines > 0 {
                let blank_lines_size = match self.squeeze.to_owned() {
                    true => 1,
                    false => consecutive_blank_lines,
                };

                let blank_lines = "\n".repeat(blank_lines_size);

                if self.syntax_highlight.clone() {
                    let ranges: Vec<(Style, &str)> = highlighter.highlight_line(&blank_lines, ps).unwrap();
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                    write!(writer, "{}", escaped).expect("Could not write to output stream");
                } else {
                    write!(writer, "{}", &blank_lines).expect("Could not write to output stream");
                }

                consecutive_blank_lines = 0;
            }

            if self.syntax_highlight.clone() {
                let ranges: Vec<(Style, &str)> = highlighter.highlight_line(&line, ps).unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                self.maybe_print_line_with_line_numbers(writer, &escaped, &num);
            } else {
                self.maybe_print_line_with_line_numbers(writer, &line, &num);
            }
            num += 1;

        }
    }
}

#[cfg(test)]
mod tests {
    use syntect::highlighting::ThemeSet;
    use tempfile::NamedTempFile;
    use crate::file_parser::test_writer::TestWriter;
    use super::*;

    #[test]
    fn test_replace_non_printing() {
        let file_parser = FileParser::new(false, false, false, true);
        let line = "hello\x0b\x0c\x0e\x0f\x1b\x7f";
        let expected = "hello^K^L^N^O^E^?";
        let actual = file_parser.replace_non_printing_characters(line);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_non_printing_no_show_non_printing() {
        let file_parser = FileParser::new(false, false, false, false);
        let line = "hello\x0b\x0c\x0e\x0f\x1b\x7f";
        let expected = "hello\x0b\x0c\x0e\x0f\x1b\x7f";
        let actual = file_parser.replace_non_printing_characters(line);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_non_printing_no_show_non_printing_with_line_numbers() {
        let file_parser = FileParser::new(true, false, false, false);
        let line = "hello\x0b\x0c\x0e\x0f\x1b\x7f";
        let expected = "1 hello\x0b\x0c\x0e\x0f\x1b\x7f";

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_for_file("test.txt").unwrap().unwrap();
        let mut highlighter = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

        let mut writer = TestWriter::new();

        file_parser.print_contents(&mut writer, &mut highlighter, &ps, line.to_string());
        writer.flush().unwrap();
        let contents = writer.into_string();
        assert_eq!(expected, contents);
    }

    #[test]
    fn test_squeeze_blank_line_option() {
        let file_parser = FileParser::new(false, true, false, false);
        let line = "line1\n\nline2\n\n\nline3";
        let expected = "line1\n\nline2\n\nline3";

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_for_file("test.txt").unwrap().unwrap();
        let mut highlighter = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

        let mut writer = TestWriter::new();

        file_parser.print_contents(&mut writer, &mut highlighter, &ps, line.to_string());

        let contents = writer.into_string();
        assert_eq!(expected, contents);
    }

    #[test]
    fn test_print_contents() {
        let file_parser = FileParser::new(false, false, false, false);
        let line = "hello";
        let expected = "hello";

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_for_file("test.txt").unwrap().unwrap();
        let mut highlighter = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

        let mut writer = TestWriter::new();

        file_parser.print_contents(&mut writer, &mut highlighter, &ps, line.to_string());

        let contents = writer.into_string();
        assert_eq!(expected, contents);
    }

    #[test]
    fn test_syntax_highlighting() {
        let file_parser = FileParser::new(true, false, true, false);
        let line = "hello";
        let expected = "1 \u{1b}[48;2;43;48;59m\u{1b}[38;2;192;197;206mhello";

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_for_file("test.txt").unwrap().unwrap();
        let mut highlighter = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

        let mut writer = TestWriter::new();

        file_parser.print_contents(&mut writer, &mut highlighter, &ps, line.to_string());

        let contents = writer.into_string();
        assert_eq!(expected, contents);
    }

    #[test]
    fn test_syntax_highlight_with_squeezing() {
        let file_parser = FileParser::new(true, true, true, false);
        let line = "hello\n\n\nworld";
        let expected = "1 \u{1b}[48;2;43;48;59m\u{1b}[38;2;192;197;206mhello\n\u{1b}[48;2;43;48;59m\u{1b}[38;2;192;197;206m\n2 \u{1b}[48;2;43;48;59m\u{1b}[38;2;192;197;206mworld";

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_for_file("test.txt").unwrap().unwrap();
        let mut highlighter = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

        let mut writer = TestWriter::new();

        file_parser.print_contents(&mut writer, &mut highlighter, &ps, line.to_string());

        let contents = writer.into_string();
        assert_eq!(expected, contents);
    }

    #[test]
    fn test_file_read_contents() {
        let tmp_file = NamedTempFile::new().unwrap();
        let mut file = File::create(tmp_file.path()).unwrap();
        file.write_all(b"hello").unwrap();

        let parser = FileParser::new(false, false, false, false);
        let contents = parser.read_contents(tmp_file.path().to_str().unwrap().to_string()).unwrap();

        assert_eq!(contents, "hello");
    }

    #[test]
    fn test_file_read_contents_fails() {
        let parser = FileParser::new(false, false, false, false);
        let contents = parser.read_contents("does_not_exist.txt".to_string());

        assert!(contents.is_err());
    }
}
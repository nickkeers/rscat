# rscat

`rscat` is a clone of the Unix `cat` command-line utility written in Rust. This project was created for learning purposes, not as a hugely serious project.

## Features

- **Numbering**: Number the non-blank output lines, starting at 1.
- **Squeeze**: Squeeze multiple adjacent empty lines, causing the output to be single spaced.
- **Show Non-Printing Characters**: Show non-printing characters, similar to the `-v` option of GNU `cat`.
- **Syntax Highlighting**: Syntax highlight the output. The language for syntax highlighting can also be specified.
- **Theme Selection**: Specify the theme to use from the 'syntect' crate. Choices: `base16-ocean.dark`, `base16-eighties.dark`, `base16-mocha.dark`, `base16-ocean.light`.

## Usage

```bash 
rscat [OPTIONS] [FILES]...
```

### Arguments
**[FILES]...**: Specify one or more files to read. If no files are specified, rscat will read from standard input.

### Options
* **-b**, **--line-numbers**: Number the non-blank output lines, starting at 1.
* **-s**, **--squeeze**: Squeeze multiple adjacent empty lines, causing the output to be single spaced.
* **-v**, **--show-non-printing**: Show non-printing characters, similar to the -v option of GNU cat.
* **-S**, **--syntax**: Syntax highlight the output. This will use the default language detection unless the -l option is used.
* **-l `<LANGUAGE>`**, **--language**: Specify the language for syntax-highlighting explicitly. Replace `<LANGUAGE>` with the name of the language, e.g., rust.
* **-t `<THEME>`**, **--theme**: Specify the theme to use from the 'syntect' crate. Replace `<THEME>` with one of the following options: base16-ocean.dark, base16-eighties.dark, base16-mocha.dark, base16-ocean.light. Default is base16-ocean.dark.
* **-h**, **--help**: Print help information.

## Tests

To run the tests, use the command: `cargo test`

## Contributing

If you would like to contribute to `rscat`, please fork the repository, make your changes, and open a pull request.

## License

This project is licensed under the [MIT License](LICENSE.txt)


Thank you for checking out `rscat`!

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::exit;
use unicode_segmentation::UnicodeSegmentation;
use std::num::NonZeroUsize;

fn fold_line<O: Write>(
    line: &[u8],
    mut indices: impl Iterator<Item = usize>,
    output: &mut O,
    max_width: NonZeroUsize,
    split_whitespace: bool,
) -> io::Result<()> {
    let mut start = 0;
    let mut width = 0;
    let mut last_word_width = 0;
    loop {
        let end = match indices.next() {
            Some(i) => i,
            None => break,
        };
        // This is located before incrementing width (and thereby finalizing that the character
        // will appear in the output) to prevent prematurely adding a newline when not necessary.
        if width == max_width.get() {
            width = 0;
            let line_end = if split_whitespace {
                if let Some(i) = line[start..end]
                    .iter()
                    .rposition(|&byte| char::from(byte).is_ascii_whitespace())
                {
                    width = last_word_width;
                    i + start + 1
                } else {
                    end
                }
            } else {
                end
            };
            output.write_all(&line[start..line_end])?;
            writeln!(output)?;
            start = line_end;
        }
        width += 1;
        if char::from(line[end]).is_ascii_whitespace() {
            last_word_width = 0;
        } else {
            last_word_width += 1;
        }
    }
    output.write_all(&line[start..])?;
    writeln!(output)?;
    output.flush()?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Graphemes,
    Chars,
    Bytes,
}
fn fold_file<I: BufRead, O: Write>(
    input: I,
    output: &mut O,
    max_width: NonZeroUsize,
    mode: Mode,
    split_whitespace: bool,
) -> io::Result<()> {
    for line in input.lines() {
        let line = line?;
        let bytes = line.as_bytes();
        match mode {
            Mode::Graphemes => fold_line(
                bytes,
                line.grapheme_indices(true).map(|(i, _)| i),
                output,
                max_width,
                split_whitespace,
            )?,
            Mode::Chars => fold_line(
                bytes,
                line.char_indices().map(|(i, _)| i),
                output,
                max_width,
                split_whitespace,
            )?,
            Mode::Bytes => fold_line(bytes, 0..line.len(), output, max_width, split_whitespace)?,
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("fnew")
        .version("1.0")
        .about("Utility to fold long lines.")
        .author("Koxiaet")
        .arg(
            Arg::with_name("width")
                .help("Width to fold at.")
                .short("w")
                .long("width")
                .default_value("80"),
        )
        .arg(
            Arg::with_name("bytes")
                .help("Count bytes not graphemes. Can cause invalid Unicode sequences.")
                .short("b")
                .long("bytes")
                .conflicts_with("chars"),
        )
        .arg(
            Arg::with_name("chars")
                .help("Count unicode characters not graphemes.")
                .short("c")
                .long("chars")
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("space")
                .help("Split at whitespaces when possible.")
                .short("s")
                .long("spaces"),
        )
        .arg(
            Arg::with_name("file")
                .help("File to read from, - is stdin.")
                .default_value("-"),
        )
        .get_matches();

    let width = matches.value_of("width").unwrap();
    let width: NonZeroUsize = match width.parse() {
        Ok(width) => width,
        Err(_) => {
            eprintln!("fnew: invalid number of columns: '{}'", width);
            exit(1);
        }
    };
    let mode = if matches.is_present("bytes") {
        Mode::Bytes
    } else if matches.is_present("chars") {
        Mode::Chars
    } else {
        Mode::Graphemes
    };
    let split_whitespace = matches.is_present("space");

    if let Err(e) = match matches.value_of("file").unwrap() {
        "-" => fold_file(
            io::stdin().lock(),
            &mut io::stdout(),
            width,
            mode,
            split_whitespace,
        ),
        filename => fold_file(
            BufReader::new(match File::open(filename) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("fnew: {}: {}", filename, e);
                    exit(1);
                }
            }),
            &mut io::stdout(),
            width,
            mode,
            split_whitespace,
        ),
    } {
        eprintln!("fnew: I/O error: {}", e);
        exit(1);
    }
}

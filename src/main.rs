use clap::{App, Arg};
use fnew::*;
use std::fs::File;
use std::io::{self, BufReader, ErrorKind};
use std::num::NonZeroUsize;
use std::process::exit;

fn main() {
    let matches = App::new("fnew")
        .version("1.0")
        .about("Utility to fold long lines.")
        .author("Kestrer")
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
        match e.kind() {
            ErrorKind::BrokenPipe => (),
            _ => {
                eprintln!("fnew: I/O error: {}", e);
                exit(1);
            }
        }
    }
}

# Fnew

## Features and Limitations

- Counts in Unicode graphemes by default.
- Can count in bytes or characters.
- Exact same syntax as GNU Fold.
- Only works with UTF-8 input.
- Line oriented. While GNU Fold buffers the entire input before folding it all at once, fnew takes
  in and prints its input line by line. This makes it suitable to be piped into from long running
  commands.

## Usage

```
USAGE:
    fnew [FLAGS] [OPTIONS] [file]

FLAGS:
    -b, --bytes      Count bytes not graphemes. Can cause invalid Unicode sequences.
    -c, --chars      Count unicode characters not graphemes.
    -h, --help       Prints help information
    -s, --spaces     Split at whitespaces when possible.
    -V, --version    Prints version information

OPTIONS:
    -w, --width <width>    Width to fold at. [default: 80]

ARGS:
    <file>    File to read from, - is stdin. [default: -]
```

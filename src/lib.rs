use std::io::{self, BufRead, Write};
use std::num::NonZeroUsize;
use unicode_segmentation::UnicodeSegmentation;

pub fn fold_line<O: Write>(
    line: &[u8],
    indices: impl Iterator<Item = usize>,
    output: &mut O,
    max_width: NonZeroUsize,
    split_whitespace: bool,
) -> io::Result<()> {
    let mut start = 0;
    let mut width = 0;
    let mut last_word_width = 0;
    for end in indices {
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
pub enum Mode {
    Graphemes,
    Chars,
    Bytes,
}

pub fn fold_file<I: BufRead, O: Write>(
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

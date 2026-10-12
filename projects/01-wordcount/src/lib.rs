//! Counting logic for `wordcount`. `main.rs` handles the CLI; everything testable lives here.

use std::io;

/// Line, word and byte counts for one input.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

impl Counts {
    /// Add another set of counts into this one (used for the `total` row).
    pub fn add(&mut self, other: Counts) {
        let _ = other;
        todo!("milestone 2: add each field of `other` into `self`")
    }
}

/// Count lines (newline characters), whitespace-separated words, and bytes in `text`.
pub fn count(text: &str) -> Counts {
    let _ = text;
    todo!("milestone 1")
}

/// Read the file at `path` and count it.
pub fn count_file(path: &str) -> io::Result<Counts> {
    let _ = path;
    todo!("milestone 3: read the file, then reuse `count`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_empty() {
        assert_eq!(count(""), Counts::default());
    }

    #[test]
    fn count_single_line_without_newline() {
        let c = count("hello world");
        assert_eq!(
            c,
            Counts {
                lines: 0,
                words: 2,
                bytes: 11
            }
        );
    }

    #[test]
    fn count_multiple_lines() {
        let c = count("one two\nthree\n\nfour five six\n");
        assert_eq!(
            c,
            Counts {
                lines: 4,
                words: 6,
                bytes: 29
            }
        );
    }

    #[test]
    fn count_handles_mixed_whitespace() {
        let c = count("  a\t\tb   c \n");
        assert_eq!(c.words, 3);
    }

    #[test]
    fn count_bytes_not_chars() {
        // "é" is two bytes in UTF-8.
        assert_eq!(count("é").bytes, 2);
    }

    #[test]
    fn add_counts() {
        let mut total = Counts {
            lines: 1,
            words: 2,
            bytes: 3,
        };
        total.add(Counts {
            lines: 10,
            words: 20,
            bytes: 30,
        });
        assert_eq!(
            total,
            Counts {
                lines: 11,
                words: 22,
                bytes: 33
            }
        );
    }

    #[test]
    fn count_file_missing_is_an_error() {
        let err = count_file("definitely/not/here.txt").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn count_file_reads_this_crates_manifest() {
        // Tests run with the crate directory as the working directory.
        let c = count_file("Cargo.toml").expect("Cargo.toml should be readable");
        assert!(c.lines > 0 && c.words > 0 && c.bytes > 0);
    }
}

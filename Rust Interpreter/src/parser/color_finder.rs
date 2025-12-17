#[path = "../tests/color_finder.rs"]
mod tests;

use super::slice::Slice;
use bstr::ByteSlice;
use regex::bytes::{Regex, RegexBuilder};

const HTML_REGEX: &str = include_str!("./html_color_regex.txt");

pub struct ColorFinder {
    regex: Regex,
}

impl ColorFinder {
    pub fn new() -> Self {
        let regex = RegexBuilder::new(HTML_REGEX)
            .case_insensitive(true)
            .unicode(false)
            .build()
            .unwrap();

        Self { regex }
    }
    //returns the color and length from start
    pub fn find<'a>(&self, slice: Slice<'a>) -> Option<(String, usize)> {
        let first_match = self.regex.find_at(slice.str, 0)?;

        let bytes = first_match.as_bytes().trim();

        Some((
            String::from_utf8(
                bytes
                    .into_iter()
                    .filter(|e| !e.is_ascii_whitespace())
                    .map(|e| e.to_ascii_lowercase())
                    .collect(),
            )
            .unwrap(),
            bytes.len(),
        ))
    }
}

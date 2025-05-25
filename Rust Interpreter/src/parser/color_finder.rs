#[path = "../tests/color_finder.rs"]
mod tests;

use super::slice::Slice;
use bstr::ByteSlice;
use regex::bytes::{RegexBuilder, RegexSet, RegexSetBuilder};

const HTML_COLORS: &str = include_str!("./html_colors.txt");

struct ColorFinder {
    colors: Vec<&'static str>,
    regex: RegexSet,
}

impl ColorFinder {
    pub fn new() -> Self {
        let colors = HTML_COLORS.split(",").collect::<Vec<_>>();

        let pattern = colors
            .iter()
            .map(|e| format!("^{}(\\s|$)", e.replace(" ", "\\s*")));

        let regex = RegexSetBuilder::new(pattern)
            .case_insensitive(true)
            .unicode(false)
            .build()
            .unwrap();

        Self { colors, regex }
    }

    pub fn find<'a>(&self, slice: Slice<'a>) -> Option<&'static str> {
        let matches = self.regex.matches_at(slice.str, 0);

        let first_match = matches.iter().next();

        first_match.map(|e| self.colors[e])
    }
}

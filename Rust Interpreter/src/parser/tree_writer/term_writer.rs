use crate::parser::ParserSource;

use super::{
    lint_writer::{LintColor, LintData},
    Paragraph, TreeAllWriter,
};

pub struct TermWriter {
    // data: &'a ParserData,
    data: Option<LintData>,
    index: usize,
}

impl TermWriter {
    pub fn new() -> Self {
        Self {
            data: None,
            index: 0,
        }
    }

    pub fn step(&mut self, tree: &Vec<Box<dyn Paragraph>>) -> bool {
        self.index += 1;
        self.data = tree
            .get(self.index - 1)
            .map(|e| TreeAllWriter::write_all_lint(e.as_ref()));
        // println!("{:?}", self.data);
        self.data.is_some()
    }

    pub fn next(&self, source: &ParserSource) -> String {
        let str = str::from_utf8(source.get_source(self.index - 1).unwrap()).unwrap();
        let mut ret = String::new();
        let mut source_index = 0;

        for (index, color) in self.data.as_ref().unwrap().get_iter() {
            ret.push_str(&str[source_index..index]);
            source_index += index - source_index;
            Self::add_convert_color(&mut ret, color);
        }

        ret.push_str(&str[source_index..]);
        ret
    }

    fn add_convert_color(str: &mut String, color: LintColor) {
        str.push_str("\x1b[");
        str.push_str(match color {
            LintColor::Ignore => "37",
            LintColor::Title => "97",
            LintColor::TitleBy => "93",
            LintColor::TitleImport => "94",
            LintColor::TitleAuthor => "96",
            LintColor::TitleSeparator => "95",
            LintColor::Alias1 => todo!(),
            LintColor::Alias2 => todo!(),
            LintColor::Alias3 => todo!(),
            LintColor::Color => todo!(),
        });
        str.push('m');
    }
}

// impl Iterator for TermWriter {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {}
// }
//             (TermColor::Black, false) => b"30",
//             (TermColor::Red, false) => b"31",
//             (TermColor::Green, false) => b"32",
//             (TermColor::Yellow, false) => b"33",
//             (TermColor::Blue, false) => b"34",
//             (TermColor::Purple, false) => b"35",
//             (TermColor::Cyan, false) => b"36",
//             (TermColor::White, false) => b"37",
//             (TermColor::Black, true) => b"90",
//             (TermColor::Red, true) => b"91",
//             (TermColor::Green, true) => b"92",
//             (TermColor::Yellow, true) => b"93",
//             (TermColor::Blue, true) => b"94",
//             (TermColor::Purple, true) => b"95",
//             (TermColor::Cyan, true) => b"96",
//             (TermColor::White, true) => b"97",

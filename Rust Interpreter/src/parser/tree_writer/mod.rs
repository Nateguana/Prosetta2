pub mod lint_writer;
pub mod term_writer;

use itertools::Itertools;
use lint_writer::{LintData, LintWriter};

use super::Paragraph;

pub trait TreeWriter {
    fn write_lisp(&self) -> String;
    fn write_lint(&self, writer: &mut LintWriter);
    fn write_javascript(&self, indent: u8) -> String;
}

pub struct TreeAllWriter;

impl TreeAllWriter {
    pub fn write_all_lisp(tree: &Vec<Box<dyn Paragraph>>) -> String {
        tree.into_iter().map(|par| par.write_lisp()).join("\n\n")
    }

    pub fn write_all_lint(paragraph: &dyn Paragraph) -> LintData {
        let mut lint_writer = LintWriter::new();
        paragraph.write_lint(&mut lint_writer);
        lint_writer.into_data()
    }

    pub fn write_all_javascript(tree: &Vec<Box<dyn Paragraph>>) -> String {
        tree.into_iter()
            .map(|par| par.write_javascript(0))
            .join("\n\n")
    }
}

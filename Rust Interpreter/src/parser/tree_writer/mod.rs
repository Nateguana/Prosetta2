pub mod indent;
pub mod lint_writer;
pub mod term_writer;

use super::Paragraph;
use crate::parser::parsable_vec::ParsableVec;
use indent::Indent;
use itertools::Itertools;
use lint_writer::{LintData, LintWriter};

pub trait TreeWriter {
    fn write_lisp(&self, vec: &ParsableVec) -> String;
    fn write_lint(&self, vec: &ParsableVec, writer: &mut LintWriter, indent: u8);
    fn write_javascript(&self, vec: &ParsableVec, indent: Indent) -> String;
}

pub struct TreeAllWriter;

impl TreeAllWriter {
    pub fn write_all_lisp(vec: &ParsableVec) -> String {
        vec.get(1).write_lisp(vec)
    }

    pub fn write_all_lint(vec: &ParsableVec) -> LintData {
        let mut lint_writer = LintWriter::new();
        vec.get(1).write_lint(vec, &mut lint_writer, 0);
        lint_writer.into_data()
    }

    pub fn write_all_javascript(vec: &ParsableVec) -> String {
        vec.get(1).write_javascript(vec, Indent::new())
    }
}

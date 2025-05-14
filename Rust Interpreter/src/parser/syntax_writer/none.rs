use crate::parser::commands::none::NoneStart;

use super::{commands::none::None, LintWriter, SyntaxWriter};

impl SyntaxWriter for None {
    fn write_lint(&self, _writer: &mut LintWriter) {}
}

impl SyntaxWriter for NoneStart {
    fn write_lint(&self, _writer: &mut LintWriter) {}
}

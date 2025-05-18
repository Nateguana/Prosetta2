use super::{commands::paragraph_start::ParagraphStart, LintWriter, SyntaxWriter};

impl SyntaxWriter for ParagraphStart {
    fn write_lint(&self, writer: &mut LintWriter) {
        let children = self.children.read();

        for child in children.iter() {
            child.write_lint(writer);
        }
    }
}

use super::{commands::title::Title, LintColor, LintWriter, SyntaxWriter};

impl SyntaxWriter for Title {
    fn write_lint(&self, writer: &mut LintWriter) {
        let this = self.inner.read();

        writer.write_up_to_as(LintColor::Title, this.by_start);
        writer.write_as(LintColor::TitleSeparator, 2);

        let mut authors = this.authors.iter().peekable();
        let mut imports = this.imports.iter().peekable();
        while let Some(author) = authors.peek() {
            writer.write_up_to(author.pos);
            let author_end = author.pos + author.length;
            while let Some(import) = imports.peek() {
                // if import before end -- write that
                if import.pos < author_end {
                    writer.write_up_to_as(LintColor::TitleAuthor, import.pos);
                    writer.write_as(LintColor::TitleImport, import.length.into());
                    imports.next();
                } else {
                    break;
                }
            }
            writer.write_up_to_as(LintColor::TitleAuthor, author_end);
            authors.next();
        }
    }
}

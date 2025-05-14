use super::{commands::title::Title, LintColor, LintWriter, SyntaxWriter};
use std::str;

impl SyntaxWriter for Title {
    fn write_lint(&self, writer: &mut LintWriter) {
        let ret = Vec::new();
        let this = self.inner.read();

        // ret.push(Lint::new(LintColor::Title, 0, this.by_start));
        // ret.push(Lint::new(LintColor::Title, this.by_start, 2));
        

        let mut authors = this.authors.iter().peekable();
        let mut imports = this.imports.iter().peekable();
        loop {
            // get lowest indexed thing or break -- this is atrocious
            let Some((write_delim, index, length)) = (match (authors.peek(), imports.peek()) {
                (None, None) => None,
                (Some(author), None) => Some((true, delim.0, delim.1.into())),
                (None, Some(author)) => Some((false, author.1, author.2)),
                (Some(delim), Some(author)) => Some(if delim.0 < author.1 {
                    (true, delim.0, delim.1.into())
                } else {
                    (false, author.1, author.2)
                }),
            }) else {
                break;
            };
        }
    }
}

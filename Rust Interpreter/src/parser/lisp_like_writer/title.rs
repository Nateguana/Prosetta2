use super::{commands::title::Title, LispWriter};
use std::str;

impl LispWriter for Title {
    fn write_lisp(&self) -> String {
        // escape potetial " in title
        let title_str = str::from_utf8(&self.title).unwrap().replace("\"", "\\\"");
        let title_length = self.title_length;

        let authors_str = self.authors.iter().fold(String::new(), |acc, data| {
            let author_str = str::from_utf8(&data.name).unwrap();
            format!("{acc} \"{author_str}\"@{}$${}", data.pos, data.length)
        });

        let imports_str = self.imports.iter().fold(String::new(), |acc, data| {
            let import_str = data.name.name();
            format!("{acc} \"{import_str}\"@{}$${}", data.pos, data.length)
        });

        format!(
            "(title \"{title_str}\"$${title_length} (authors{authors_str}) (imports{imports_str}))",
        )
    }
}

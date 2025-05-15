use super::{commands::title::Title, LispWriter};
use std::str;

impl LispWriter for Title {
    fn write_lisp(&self) -> String {
        let this = self.inner.read();
        // escape potetial " in title
        let title_str = str::from_utf8(&this.title).unwrap().replace("\"", "\\\"");
        let title_length = this.title_length;
        let by_section_length = this.by_section_length;

        if by_section_length == 0 {
            format!("(title \"{title_str}\"$${title_length})")
        } else {
            let authors_str = this.authors.iter().fold(String::new(), |acc, data| {
                let author_str = str::from_utf8(&data.name).unwrap();
                format!("{acc} \"{author_str}\"@{}$${}", data.pos, data.length)
            });

            let imports_str = this.imports.iter().fold(String::new(), |acc, data| {
                let import_str = data.name.name();
                format!("{acc} \"{import_str}\"@{}$${}", data.pos, data.length)
            });
            format!(
            "(title \"{title_str}\" (by${title_length}$${by_section_length} (authors{authors_str}) (imports{imports_str})))",
            )
        }
    }
}

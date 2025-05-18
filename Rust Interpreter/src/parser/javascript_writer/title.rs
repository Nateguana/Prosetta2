use super::{commands::title::Title, JavascriptWriter};
use itertools::Itertools;
use std::str;

impl JavascriptWriter for Title {
    fn write_javascript(&self, _indent: u8) -> String {
        let this = self.inner.read();
        let title_str = str::from_utf8(&this.title).unwrap();
        let mut authors = this
            .authors
            .iter()
            .map(|e| str::from_utf8(&e.name).unwrap());

        let primary_author_str = {
            authors
                .next()
                .map_or("".to_string(), |name| format!("\nPrimary Author: {name}"))
        };

        let secondary_authors_str = {
            let secondary_authors = authors.collect::<Vec<_>>();
            match secondary_authors.len() {
                0 => "".to_string(),
                len => format!(
                    "\nSecondary Author{}: {}",
                    if len > 1 { "s" } else { "" },
                    secondary_authors.join(", ")
                ),
            }
        };

        let imports_str = {
            let mut imports = this.imports.iter().map(|e| e.import.name());
            match imports.len() {
                0 => "".to_string(),
                len => format!(
                    "\nImport{}: {}",
                    if len > 1 { "s" } else { "" },
                    imports.join(", ")
                ),
            }
        };

        format!(
            "/*\nTitle: {title_str}{primary_author_str}{secondary_authors_str}{imports_str}\n*/",
        )
    }
}

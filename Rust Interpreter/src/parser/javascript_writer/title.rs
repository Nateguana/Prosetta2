use itertools::Itertools;
use std::str;

use super::commands::title::Title;

use super::JavascriptWriter;

impl JavascriptWriter for Title {
    fn write(&self) -> String {
        let title_str = str::from_utf8(&self.title).unwrap();
        let mut authors = self
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
            let mut imports = self.imports.iter().map(|e| e.name.name());
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
            "/*
            Title: {title_str}{primary_author_str}{secondary_authors_str}{imports_str}
            */",
        )
    }
}

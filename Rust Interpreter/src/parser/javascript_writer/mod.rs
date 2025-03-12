mod none;
mod paragraph_type;
mod title;
use itertools::Itertools;

use super::{commands, Paragraph, ParagraphType};

pub trait JavascriptWriter {
    fn write_javascript(&self) -> String;
}

pub fn write_all(tree: &Vec<Paragraph>) -> String {
    tree.into_iter()
        .map(|par| par.data.write_javascript())
        .join("\n\n")
}

mod none;
mod title;
mod paragraph_start;

use itertools::Itertools;

use super::{commands, Paragraph};

pub trait JavascriptWriter {
    fn write_javascript(&self, indent: u8) -> String;
}

pub fn write_all(tree: &Vec<Box<dyn Paragraph>>) -> String {
    tree.into_iter()
        .map(|par| par.write_javascript(0))
        .join("\n\n")
}

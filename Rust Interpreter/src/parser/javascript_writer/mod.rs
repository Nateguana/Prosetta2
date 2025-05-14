mod none;
mod title;
use itertools::Itertools;

use super::{commands, Paragraph};

pub trait JavascriptWriter {
    fn write_javascript(&self) -> String;
}

pub fn write_all(tree: &Vec<Box<dyn Paragraph>>) -> String {
    tree.into_iter()
        .map(|par| par.write_javascript())
        .join("\n\n")
}

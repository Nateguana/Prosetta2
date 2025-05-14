mod none;
mod title;

use super::{commands, Paragraph};
use itertools::Itertools;

pub trait LispWriter {
    fn write_lisp(&self) -> String;
}

pub fn write_all(tree: &Vec<Box<dyn Paragraph>>) -> String {
    tree.into_iter()
        .map(|par| par.write_lisp())
        .join("\n\n")
}

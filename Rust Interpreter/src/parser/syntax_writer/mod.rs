mod none;
mod title;

use super::{commands, Paragraph};
use itertools::{Itertools, Position};

pub(crate) mod term_writer;

#[derive(Debug, PartialEq, Clone, Copy)]
enum LintColor {
    Ignore,
    Title,
    TitleImport,
    TitleAuthor,
    TitleSeparator,
}

// pub struct Lint {
//     position: usize,
//     // size: usize,
//     color: LintColor,
// }

// impl Lint {
//     pub fn new(color: LintColor, position: usize, size: usize) -> Self {
//         Self {
//             position,
//             color,
//         }
//     }
// }

pub struct LintWriter {
    positions: Vec<usize>,
    colors: Vec<LintColor>,
    last_index: usize,
    last_color: LintColor,
}

impl LintWriter {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            colors: Vec::new(),
            last_index: 0,
            last_color: LintColor::Ignore,
        }
    }

    pub fn write_up_to(&mut self, index: usize) {
        self.write_up_to_as(LintColor::Ignore, index);
    }

    pub fn write_up_to_as(&mut self, color: LintColor, index: usize) {
        if index < self.last_index {
            panic!(
                "index {} should be after the writing index {}",
                index, self.last_index
            )
        } else {
            self.push_color(color, index);
        }
    }

    pub fn write_as(&mut self, color: LintColor, num: usize) {
        // self.last_index += num;
        self.push_color(color, self.last_index + num);
    }

    pub fn finish(&mut self) {
        // if self.last_color != LintColor::Ignore {
        //     self.positions.push(self.last_index);
        //     self.colors.push(LintColor::Ignore);
        // }
        self.push_color(LintColor::Ignore, self.last_index + 1);
    }

    fn push_color(&mut self, color: LintColor, index: usize) {
        if self.last_color != color {
            self.positions.push(self.last_index);
            self.colors.push(color);
        }
        self.last_index = index;
        self.last_color = color;
    }
}

pub trait SyntaxWriter {
    fn write_lint(&self, writer: &mut LintWriter);
}

pub fn write(paragraph: &dyn Paragraph) -> LintWriter {
    let mut lint_writer = LintWriter::new();
    paragraph.write_lint(&mut lint_writer);
    lint_writer
}

// pub fn write_all(tree: &Vec<Box<dyn Paragraph>>) -> Vec<Vec<Lint>> {
//     tree.into_iter().map(|par| write(par))
// }

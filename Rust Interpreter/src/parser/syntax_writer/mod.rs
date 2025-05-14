mod none;
mod title;

use super::{commands, Paragraph};
use itertools::{Itertools, Position};

pub(crate) mod term_writer;

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
    index: usize,
}

impl LintWriter {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            colors: Vec::new(),
            index: 0,
        }
    }

    fn write_up_to(&mut self, index: usize) {

    }

    fn write_up_to_as(&mut self, color: LintColor, index: usize) {
        let num = index.checked_sub(self.index).expect(&format!(
            "index {} should be after the writing index {}",
            index, self.index
        ));
        let buf = Self::get_n_or_error(source, num);
        self.renderer.add_with(&buf, color);
        self.index = index;
    }
    // fn write_num(&mut self, source: &mut ParserSourceIter, index: usize) {
    //     self.write_as(source, index, BASE_COLOR);
    // }
    fn write_as(&mut self, source: &mut ParserSourceIter, num: usize, color: (TermColor, bool)) {
        let buf: Vec<u8> = Self::get_n_or_error(source, num);
        self.renderer.add_with(&buf, color);
        self.index += num;
    }

    fn write_end(&mut self, source: &mut ParserSourceIter) {
        if let Some(end) = self.ends.take() {
            // let num = index
            //     .checked_sub(self.index)
            //     .expect("index is before the end index");
            let buf = Self::get_n_or_error(source, end.0 as usize);
            self.renderer.add_with_mult(&buf, end.1);
            self.index += end.0 as usize;
        }
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

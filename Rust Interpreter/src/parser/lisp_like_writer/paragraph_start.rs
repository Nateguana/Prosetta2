use super::{commands::paragraph_start::ParagraphStart, LispWriter};
use std::str;

impl LispWriter for ParagraphStart {
    fn write_lisp(&self) -> String {
        let children = self.children.read();
        let index = self.index;

        let children_str = children.iter().fold(String::new(), |acc, data| {
            let child_str = data.write_lisp();
            format!("{acc} {child_str}")
        });

        format!("(paragraph {index}{children_str})")
    }
}

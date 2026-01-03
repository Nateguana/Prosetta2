use super::{Context, Indent, LintWriter, Paragraph, Parsable, ParseTreeObj, ReturnType, Slice};
use crate::parser::{context::ParseResult, parsable_vec::ParsableVec, tree_writer::TreeWriter};
use std::any::Any;

#[derive(Debug)]
pub struct ParagraphStart {
    pub children: Vec<usize>,
    pub index: usize,
}

impl ParagraphStart {
    pub fn new(index: usize) -> Self {
        Self {
            children: Default::default(),
            index,
        }
    }

    pub fn parse(&self, co: Context, _slice: Slice) -> ParseResult {
        co.result_match(0, ReturnType::Null)
    }
}

impl ParseTreeObj for ParagraphStart {
    fn name(&self) -> &'static str {
        "Paragraph"
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// #[async_trait::async_trait]
impl Parsable for ParagraphStart {
    fn get_children(&self) -> Vec<usize> {
        self.children.clone()
    }
}

impl Paragraph for ParagraphStart {
    fn get_index(&self) -> usize {
        self.index
    }

    // fn get_children(&self) -> RwLockReadGuard<'_, Vec<Box<dyn Stat>>> {
    //     self.children.read()
    // }
}

impl TreeWriter for ParagraphStart {
    fn write_lisp(&self, vec: &ParsableVec) -> String {
        let index = self.index;

        let children_str = self.children.iter().fold(String::new(), |acc, &index| {
            let child_str = vec.get(index).write_lisp(vec);
            format!("{acc} {child_str}")
        });

        format!("(paragraph {index}{children_str})")
    }

    fn write_lint(&self, vec: &ParsableVec, writer: &mut LintWriter, indent: u8) {
        for &index in self.children.iter() {
            vec.get(index).write_lint(vec, writer, indent);
        }
    }

    fn write_javascript(&self, vec: &ParsableVec, indent: Indent) -> String {
        let mut ret = format!("// Paragraph {}", self.index);

        for &index in self.children.iter() {
            ret += &format!("\n{}", vec.get(index).write_javascript(vec, indent));
        }

        ret
    }
}

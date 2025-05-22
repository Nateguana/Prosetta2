use std::any::Any;

use crate::parser::tree_writer::TreeWriter;

use super::{
    Context, FailReason, LintWriter, Paragraph, Parsable, ParseTreeObj, ReturnType, RwLock, Slice,
    Stat,
};

#[derive(Debug)]
pub struct ParagraphStart {
    pub children: RwLock<Vec<Box<dyn Stat>>>,
    pub index: usize,
}

impl ParagraphStart {
    pub fn new(index: usize) -> Self {
        Self {
            children: Default::default(),
            index,
        }
    }
}

impl ParseTreeObj for ParagraphStart {
    fn name(&self) -> &'static str {
        "Title"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait::async_trait]
impl Parsable for ParagraphStart {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        Ok((slice.end(), ReturnType::Null))
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
    fn write_lisp(&self) -> String {
        let children = self.children.read();
        let index = self.index;

        let children_str = children.iter().fold(String::new(), |acc, data| {
            let child_str = data.write_lisp();
            format!("{acc} {child_str}")
        });

        format!("(paragraph {index}{children_str})")
    }

    fn write_lint(&self, writer: &mut LintWriter) {
        let children = self.children.read();

        for child in children.iter() {
            child.write_lint(writer);
        }
    }

    fn write_javascript(&self, _indent: u8) -> String {
        let children = self.children.read();
        let mut ret = format!("// Paragraph {}", self.index);

        for child in children.iter() {
            ret += &format!("\n{}", child.write_javascript(0));
        }

        ret
    }
}

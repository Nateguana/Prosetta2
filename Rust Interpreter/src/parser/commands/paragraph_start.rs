use std::any::Any;

use super::{
    close_data, CloseData, Context, FailReason, Import, ImportData, ImportFinder, Paragraph,
    Parsable, ParseTreeObj, ReturnType, RwLock, RwLockReadGuard, Slice, Stat, Step_Continue,
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

    fn get_children(&self) -> RwLockReadGuard<'_, Vec<Box<dyn Stat>>> {
        self.children.read()
    }
}

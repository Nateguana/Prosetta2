use super::{
    Command, Context, FailReason, Indent, LintWriter, Parsable, ParseTreeObj, ReturnType,
    ReturnTypeSet, RwLockMappedWriteGuard, Slice, TreeWriter,
};
use std::any::Any;

#[derive(Debug)]
pub struct NoneCommand;

impl NoneCommand {
    pub fn new() -> Self {
        Self
    }
}

impl ParseTreeObj for NoneCommand {
    fn name(&self) -> &'static str {
        "None"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Parsable for NoneCommand {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        Ok((slice.end(), ReturnType::Null))
    }
}
use std::any::Any;

use super::{
    Command, Context, FailReason, Indent, LintWriter, Parsable, ParseTreeObj, ReturnType,
    ReturnTypeSet, Slice, TreeWriter,
};

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

#[async_trait::async_trait]
impl Parsable for NoneCommand {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        Ok((slice.end(), ReturnType::Null))
    }
}

impl Command for NoneCommand {
    fn get_return_types(&self) -> ReturnTypeSet {
        ReturnTypeSet::Any
    }
}

impl TreeWriter for NoneCommand {
    fn write_lisp(&self) -> String {
        format!("(TODO)")
    }
    fn write_lint(&self, _writer: &mut LintWriter, _indent: u8) {}
    fn write_javascript(&self, _indent: Indent) -> String {
        format!("TODO()")
    }
}
#[derive(Debug)]
pub struct NoneStart;

impl NoneStart {
    pub fn new() -> Self {
        Self
    }
}

impl ParseTreeObj for NoneStart {
    fn name(&self) -> &'static str {
        "Start"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TreeWriter for NoneStart {
    fn write_lisp(&self) -> String {
        unreachable!()
    }

    fn write_lint(&self, _writer: &mut LintWriter, _indent: u8) {}

    fn write_javascript(&self, _indent: Indent) -> String {
        unreachable!()
    }
}

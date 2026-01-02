use super::{
    Command, Indent, LintWriter, Parsable, ParsableVec, ParseTreeObj, ReturnTypeSet, TreeWriter,
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Parsable for NoneCommand {
    fn get_children(&self) -> Vec<usize> {
        unreachable!()
    }
}

impl Command for NoneCommand {
    fn get_return_types(&self) -> ReturnTypeSet {
        ReturnTypeSet::Any
    }
}

impl TreeWriter for NoneCommand {
    fn write_lisp(&self, _vec: &ParsableVec) -> String {
        format!("(TODO)")
    }
    fn write_lint(&self, _vec: &ParsableVec, _writer: &mut LintWriter, _indent: u8) {}
    fn write_javascript(&self, _vec: &ParsableVec, _indent: Indent) -> String {
        format!("TODO()")
    }
}

use std::any::Any;

use crate::parser::tree_writer::TreeWriter;

use super::{
    LintWriter, ParseTreeObj,
};

#[derive(Debug)]
pub struct None;

impl None {
    fn new() -> Self {
        Self
    }
}

impl ParseTreeObj for None {
    fn name(&self) -> &'static str {
        "None"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TreeWriter for None {
    fn write_lisp(&self) -> String {
        format!("(TODO)")
    }
    fn write_lint(&self, _writer: &mut LintWriter) {}
    fn write_javascript(&self, _indent: u8) -> String {
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

    fn write_lint(&self, _writer: &mut LintWriter) {}

    fn write_javascript(&self, _indent: u8) -> String {
        unreachable!()
    }
}

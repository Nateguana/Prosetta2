use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use std::{any::Any, fmt::format, mem, ops::Add};
// use parking_lot::{Mutex, MutexGuard};

use crate::parser::tree_writer::TreeWriter;

use super::{
    none::NoneCommand, AliasName, Aliased, Command, Context, FailReason, Parsable, ParseTreeObj,
    ReturnType, ReturnTypeSet, RwLock, Slice,
};


#[derive(Debug)]
pub struct Color {
    pub color: Vec<u8>,
}

impl Color {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(ColorData {
                children: vec![Box::new(NoneCommand::new()), Box::new(NoneCommand::new())],
            }),
        }
    }
}

impl ParseTreeObj for Color {
    fn name(&self) -> &'static str {
        "Color"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Command for Color {
    fn get_return_types(&self) -> ReturnTypeSet {
        ReturnTypeSet::Color 
    }
}

#[async_trait::async_trait]
impl Parsable for Color {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        Ok((slice.end(), ReturnType::Null))
    }
}

// impl Expr for Color {

//     fn get_children(&self) -> RwLockReadGuard<'_, Vec<Box<dyn Stat>>> {
//        self.inner
//     }
// }

impl TreeWriter for Color {
    fn write_lisp(&self) -> String {
        let this = self.inner.read();
        let str = this
            .children
            .iter()
            .fold(String::new(), |acc, ele| acc + " " + &ele.write_lisp());
        format!("(add${}{str})", 1)
    }

    fn write_lint(&self, writer: &mut crate::parser::tree_writer::lint_writer::LintWriter) {
        todo!()
    }

    fn write_javascript(&self, indent: u8) -> String {
        let this = self.inner.read();
        let mut str = String::new();
        let mut sep = "";

        for child in this.children.iter() {
            str += &format!("{}({})", sep, child.write_javascript(indent));
            sep = " + ";
        }

        str
    }
}

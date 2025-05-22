use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use std::{any::Any, mem};
// use parking_lot::{Mutex, MutexGuard};

use crate::parser::tree_writer::TreeWriter;

use super::{
    AliasName, Aliased, Context, Expr, FailReason, Parsable, ParseTreeObj, ReturnType, RwLock, Slice,
};

#[derive(Default, Debug)]
pub struct AdditionData {
    pub children: Vec<Box<dyn Expr>>,
}

#[derive(Debug)]
pub struct Addition {
    pub inner: RwLock<AdditionData>,
}

impl Addition {
    pub fn new(index: usize) -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl ParseTreeObj for Addition {
    fn name(&self) -> &'static str {
        "Addition"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Aliased for Addition {
    fn alias(&self) -> AliasName {
        *b"add"
    }
}

#[async_trait::async_trait]
impl Parsable for Addition {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        Ok((slice.end(), ReturnType::Null))
    }
}

// impl Expr for Addition {

//     fn get_children(&self) -> RwLockReadGuard<'_, Vec<Box<dyn Stat>>> {
//        self.inner
//     }
// }

impl TreeWriter for Addition {
    fn write_lisp(&self) -> String {
        todo!()
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

        format!("()+()",)
    }
}

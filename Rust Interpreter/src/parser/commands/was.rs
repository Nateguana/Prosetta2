use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use std::{any::Any, fmt::format, mem, ops::Add};
// use parking_lot::{Mutex, MutexGuard};

use crate::parser::tree_writer::lint_writer::LintWriter;

use super::{
    none::NoneCommand, AliasLoc, AliasName, Aliased, Command, Context, FailReason, Indent,
    Parsable, ParseTreeObj, ReturnType, ReturnTypeSet, RwLock, Slice, TreeWriter,
};

#[derive(Debug)]
pub struct WasData {
    pub child: Box<dyn Command>,
    pub loc: AliasLoc,
}

#[derive(Debug)]
pub struct Was {
    pub inner: RwLock<WasData>,
}

impl ParseTreeObj for Was {
    fn name(&self) -> &'static str {
        "Was"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Command for Was {
    fn get_return_types(&self) -> ReturnTypeSet {
        ReturnTypeSet::Number | ReturnTypeSet::String
    }
}

impl Aliased for Was {
    fn new(loc: AliasLoc) -> Self {
        Self {
            inner: RwLock::new(WasData {
                child: Box::new(NoneCommand::new()),
                loc,
            }),
        }
    }

    fn alias() -> AliasName {
        *b"add"
    }

    fn get_alias(&self) -> AliasName {
        Self::alias()
    }
}

#[async_trait::async_trait]
impl Parsable for Was {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        Ok((slice.end(), ReturnType::Null))
    }
}

// impl Expr for Was {

//     fn get_children(&self) -> RwLockReadGuard<'_, Vec<Box<dyn Stat>>> {
//        self.inner
//     }
// }

impl TreeWriter for Was {
    fn write_lisp(&self) -> String {
        let this = self.inner.read();
        format!("(was)")
    }

    fn write_lint(&self, writer: &mut LintWriter, indent: u8) {
        todo!()
    }

    fn write_javascript(&self, indent: Indent) -> String {
        let this = self.inner.read();
        let mut str = String::new();
        let mut sep = "";

        str
    }
}

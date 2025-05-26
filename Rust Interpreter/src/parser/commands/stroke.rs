use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use std::{any::Any, fmt::format, mem, ops::Add};
// use parking_lot::{Mutex, MutexGuard};

use super::{
    none::NoneCommand, AliasLoc, AliasName, Aliased, Command, Context, FailReason, Parsable,
    ParseTreeObj, ReturnType, ReturnTypeSet, RwLock, Slice, TreeWriter,
};

#[derive(Debug)]
pub struct StrokeData {
    pub child: Box<dyn Command>,
    pub optional: Option<[Box<dyn Command>; 2]>,
    pub loc: AliasLoc,
}

#[derive(Debug)]
pub struct Stroke {
    pub inner: RwLock<StrokeData>,
}

impl ParseTreeObj for Stroke {
    fn name(&self) -> &'static str {
        "Stroke"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Command for Stroke {
    fn get_return_types(&self) -> ReturnTypeSet {
        ReturnTypeSet::Void
    }
}

impl Aliased for Stroke {
    fn new(loc: AliasLoc) -> Self {
        Self {
            inner: RwLock::new(StrokeData {
                child: Box::new(NoneCommand::new()),
                optional: None,
                loc,
            }),
        }
    }

    fn alias() -> AliasName {
        *b"sto"
    }

    fn get_alias(&self) -> AliasName {
        Self::alias()
    }
}

#[async_trait::async_trait]
impl Parsable for Stroke {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        Ok((slice.end(), ReturnType::Void))
    }
}

// impl Expr for Stroke {

//     fn get_children(&self) -> RwLockReadGuard<'_, Vec<Box<dyn Stat>>> {
//        self.inner
//     }
// }

impl TreeWriter for Stroke {
    fn write_lisp(&self) -> String {
        let this = self.inner.read();

        // if let Some(arr) = this.optional {
        //     format!("(stroke{} {str})", 1)
        // } else {
        //     format!("(stroke{} {str})", 1)
        // }
        "".to_string()
    }

    fn write_lint(&self, writer: &mut crate::parser::tree_writer::lint_writer::LintWriter) {
        todo!()
    }

    fn write_javascript(&self, indent: u8) -> String {
        let this = self.inner.read();
        let mut str = String::new();
        let mut sep = "";

        // for child in this.children.iter() {
        //     str += &format!("{}({})", sep, child.write_javascript(indent));
        //     sep = " + ";
        // }

        str
    }
}

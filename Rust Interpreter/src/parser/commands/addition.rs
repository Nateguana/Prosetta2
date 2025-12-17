use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use std::{any::Any, fmt::format, mem, ops::Add};
// use parking_lot::{Mutex, MutexGuard};

use super::{
    none::NoneCommand, AliasLoc, AliasName, Aliased, Command, Context, FailReason, Indent,
    LintWriter, Parsable, ParseTreeObj, ReturnType, ReturnTypeSet, RwLock, RwLockMappedWriteGuard,
    Slice, TreeWriter,
};

#[derive(Debug)]
pub struct AdditionData {
    pub children: Vec<Box<dyn Command>>,
    pub loc: AliasLoc,
}

#[derive(Debug)]
pub struct Addition {
    pub inner: RwLock<AdditionData>,
}

// impl Addition {
//     pub fn new() -> Self {
//         Self {
//             inner: RwLock::new(AdditionData {
//                 children: vec![Box::new(NoneCommand::new()), Box::new(NoneCommand::new())],
//             }),
//         }
//     }
// }

impl ParseTreeObj for Addition {
    fn name(&self) -> &'static str {
        "Addition"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Command for Addition {
    fn get_return_types(&self) -> ReturnTypeSet {
        ReturnTypeSet::Number | ReturnTypeSet::String
    }
    fn get_child<'a>(&'a self, index: usize) -> RwLockMappedWriteGuard<'a, dyn Command + 'static> {
        self.inner.write_map(|f| f.children[index].as_mut())
    }
    fn len(&self) -> usize {
        let this = self.inner.read();
        this.children.len()
    }
}

impl Aliased for Addition {
    fn new(loc: AliasLoc) -> Self {
        Self {
            inner: RwLock::new(AdditionData {
                children: vec![Box::new(NoneCommand::new()), Box::new(NoneCommand::new())],
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
        let this = self.inner.read();
        let str = this
            .children
            .iter()
            .fold(String::new(), |acc, ele| acc + " " + &ele.write_lisp());
        format!("(add${}{str})", 1)
    }

    fn write_lint(&self, _writer: &mut LintWriter, _indent: u8) {
        todo!()
    }

    fn write_javascript(&self, indent: Indent) -> String {
        let this = self.inner.read();
        let mut str = String::new();
        let mut sep = "";

        for child in this.children.iter() {
            str += &format!("{}({})", sep, child.write_javascript(indent.add()));
            sep = " + ";
        }

        str
    }
}

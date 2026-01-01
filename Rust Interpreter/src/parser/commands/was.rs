use super::{
    none::NoneCommand, AliasLoc, AliasName, Aliased, Command, Context, FailReason, Indent,
    Parsable, ParseTreeObj, ReturnType, ReturnTypeSet, RwLock, RwLockMappedWriteGuard, Slice,
    TreeWriter,
};
use crate::parser::tree_writer::lint_writer::LintWriter;
use std::{any::Any, fmt::format};

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
    fn get_child<'a>(&'a self, _index: usize) -> RwLockMappedWriteGuard<'a, dyn Command + 'static> {
        self.inner.write_map(|f| f.child.as_mut())
    }
    fn len(&self) -> usize {
        1
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
    async fn parse(
        &self,
        _co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        Ok((slice.end(), ReturnType::Null))
    }
}

impl TreeWriter for Was {
    fn write_lisp(&self) -> String {
        let this = self.inner.read();
        format!("(was{} {})", this.loc.write_lisp(), this.child.write_lisp())
    }

    fn write_lint(&self, writer: &mut LintWriter, indent: u8) {
        let this = self.inner.read();
        this.loc.write_lint(writer, indent);
        this.child.write_lint(writer, indent);
    }

    fn write_javascript(&self, indent: Indent) -> String {
        // let this = self.inner.read();

        // format!("{}let _")
        String::new()
    }
}

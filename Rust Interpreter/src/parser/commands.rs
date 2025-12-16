pub mod none;
pub mod paragraph_start;
pub mod title;

pub mod addition;
pub mod color;
pub mod print;
pub mod stroke;
pub mod subtract;
pub mod was;

use std::{any::Any, fmt::Debug};

#[allow(unused)]
use super::{
    alias_finder::AliasLoc,
    close_data::{self, CloseData},
    context::{Context, Step_Continue},
    fail_reason::FailReason,
    import_finder::ImportFinder,
    imports::{Import, ImportData},
    rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    slice::Slice,
    tree_writer::{
        indent::Indent,
        lint_writer::{LintColor, LintWriter},
        TreeWriter,
    },
    types::{ReturnType, ReturnTypeSet},
};
// use super::{child_vec::ChildVec, syntax_writer::SyntaxWriter};

pub type AliasName = [u8; 3];

#[async_trait::async_trait]
pub trait ParseTreeObj: Sync + Send + Any + Debug {
    // fn new() -> Self
    // where
    //     Self: Sized;
    fn name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;

    fn get_name(&self) -> String {
        self.name().to_string()
    }

    // fn get_children(&self);
}
#[async_trait::async_trait]
pub trait Parsable: ParseTreeObj + TreeWriter {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason>
    where
        Self: Sized;
}

#[async_trait::async_trait]
pub trait Command: Parsable {
    fn get_return_types(&self) -> ReturnTypeSet;
}

pub trait Aliased: Command {
    fn new(loc: AliasLoc) -> Self
    where
        Self: Sized;

    fn alias() -> AliasName
    where
        Self: Sized;

    fn get_alias(&self) -> AliasName;

    fn get_name(&self) -> String {
        format!(
            "{} ({})",
            str::from_utf8(&self.get_alias()).unwrap(),
            self.name().to_string()
        )
    }
}

// #[async_trait::async_trait]
// pub trait Stat: Parsable {}

// #[async_trait::async_trait]
// pub trait CommandData: Sync + Send + Any {
//     fn new() -> Self
//     where
//         Self: Sized;
// }

// // #[derive(Debug)]
// enum CommandReturnResult {
//     /// returned to go to the parent state with the index to now parse from and whether the state closed on it
//     Matched(usize, ReturnType),
//     /// returned to add a child onto the stack with an index and the state to continue with
//     // AddChild(usize, usize),
//     /// returned to give the same state with the offset (usually 0)
//     // Continue(usize),
//     /// returned to go to the parent state with a failure
//     Failed(FailReason),
// }
#[async_trait::async_trait]
pub trait Paragraph: Parsable {
    fn get_index(&self) -> usize;
    // fn get_children(&self) -> ChildVec<'_, dyn Stat>;
}

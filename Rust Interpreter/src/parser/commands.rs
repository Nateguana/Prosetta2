pub mod none;
pub mod title;

use std::{any::Any, fmt::Debug};

use super::syntax_writer::SyntaxWriter;
#[allow(unused)]
use super::{
    close_data::{self, CloseData},
    context::{Context, Step_Continue},
    fail_reason::FailReason,
    imports::Import,
    javascript_writer::JavascriptWriter,
    lisp_like_writer::LispWriter,
    rwlock::{RwLock, RwLockWriteGuard, RwLockWriteGuardArc},
    slice::Slice,
    types::{ReturnType, ReturnTypeSet},
};

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
}
#[async_trait::async_trait]
pub trait Parsable: ParseTreeObj + JavascriptWriter + LispWriter + SyntaxWriter {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason>
    where
        Self: Sized;
}

pub trait Aliased: Parsable {
    fn alias(&self) -> AliasName;

    fn get_name(&self) -> String {
        format!(
            "{} ({})",
            str::from_utf8(&self.alias()).unwrap(),
            self.name().to_string()
        )
    }
}

#[async_trait::async_trait]
pub trait Stat: Parsable {
    fn get_return_types(&self) -> ReturnTypeSet;

    fn is_none(&self) -> bool {
        false
    }
}

#[async_trait::async_trait]
pub trait Expr: Parsable {
    fn get_return_types(&self) -> ReturnTypeSet;

    fn is_none(&self) -> bool {
        false
    }
}

#[async_trait::async_trait]
pub trait CommandData: Sync + Send + Any {
    fn new() -> Self
    where
        Self: Sized;
}

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
pub trait Paragraph: Parsable {}

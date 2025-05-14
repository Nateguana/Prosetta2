use std::any::Any;

use super::{Context, FailReason, Parsable, ParseTreeObj, ReturnType, ReturnTypeSet, Slice};

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

// #[async_trait::async_trait]
// impl Parsable for None {
//     async fn try_parse(
//         &self,
//         _co: impl Context,
//         _slice: Slice<'_>,
//     ) -> Result<(usize, ReturnType), FailReason> {
//         unreachable!()
//     }
// }

// impl Expr {
//     fn is_none(&self) -> bool {
//         true
//     }

//     fn get_return_types(&self) -> ReturnTypeSet {
//         unreachable!()
//     }
// }

#[derive(Debug)]
pub struct NoneStart;

impl NoneStart {
    fn new() -> Self {
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

// #[async_trait::async_trait]
// impl Command for NoneStart {
//     async fn try_parse(
//         &mut self,
//         _co: impl Context,
//         _slice: Slice<'_>,
//     ) -> Result<(usize, ReturnType), FailReason> {
//         unreachable!()
//     }

//     fn is_none(&self) -> bool {
//         true
//     }
// }

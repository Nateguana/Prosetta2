use std::any::Any;

use super::{Command, Context, FailReason, Parseable, ReturnType, Slice};

#[derive(Debug)]
pub struct None;

impl Parseable for None {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &'static str {
        "Start"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait::async_trait]
impl Command for None {
    async fn try_parse(
        &self,
        _co: impl Context,
        _slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        unreachable!()
    }

    fn is_none(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct NoneStart;

impl Parseable for NoneStart {
    fn new() -> Self {
        Self
    }

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

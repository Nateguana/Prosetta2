use super::{Command, Context, FailReason, ReturnType, Slice};

pub struct None;

#[async_trait::async_trait]
impl Command for None {
    fn new() -> Self {
        None
    }
    async fn try_parse(
        &mut self,
        _co: &Context,
        _slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        unreachable!()
    }

    fn name(&self) -> &'static str {
        "None"
    }

    fn is_none(&self) -> bool {
        true
    }
}
pub struct Base;

#[async_trait::async_trait]
impl Command for Base {
    fn new() -> Self {
        Base
    }
    async fn try_parse(
        &mut self,
        _co: &Context,
        _slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        unreachable!()
    }

    fn name(&self) -> &'static str {
        "Base"
    }

    fn is_none(&self) -> bool {
        true
    }
}

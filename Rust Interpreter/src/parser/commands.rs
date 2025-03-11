mod alias_finder;
pub mod none;
pub mod title;

use super::{
    close_data::{self, CloseData},
    context::Context,
    fail_reason::FailReason,
    imports::Import,
    slice::Slice,
    types::ReturnType,
};

pub type AliasName = [u8; 3];

#[async_trait::async_trait]
pub trait Command: Sync + Send {
    fn new() -> Self
    where
        Self: Sized;

    async fn try_parse(
        &mut self,
        co: &Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason>;

    fn name(&self) -> &'static str;
    fn is_none(&self) -> bool {
        false
    }
}

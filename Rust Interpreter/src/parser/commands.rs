mod alias_finder;
pub mod none;
pub mod title;

use std::{any::Any, cell::RefCell};

use super::{
    close_data::{self, CloseData},
    comm_ptr::CommPtr,
    context::Context,
    fail_reason::FailReason,
    imports::Import,
    javascript_writer::JavascriptWriter,
    lisp_like_writer::LispWriter,
    slice::Slice,
    types::ReturnType,
};

pub type AliasName = [u8; 3];

#[async_trait::async_trait]
pub trait Command: Sync + Send + JavascriptWriter + LispWriter + Any {
    fn new() -> Self
    where
        Self: Sized;

    async fn try_parse(
        self: CommPtr<'_, Self>,
        co: &Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason>
    where
        Self: Sized;

    fn name(&self) -> &'static str;
    fn is_none(&self) -> bool {
        false
    }
}

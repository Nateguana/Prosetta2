use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use std::{any::Any, fmt::format, mem, ops::Add};
// use parking_lot::{Mutex, MutexGuard};

use super::{
    none::NoneCommand, AliasName, Aliased, Command, Context, FailReason, LintColor, LintWriter,
    Parsable, ParseTreeObj, ReturnType, ReturnTypeSet, RwLock, Slice, TreeWriter,
};

#[derive(Debug)]
pub struct ColorData {
    pub color: String,
    pub pos: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct Color {
    pub inner: RwLock<ColorData>,
}

impl Color {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(ColorData {
                color: String::new(),
                pos: 0,
                length: 0,
            }),
        }
    }
}

impl ParseTreeObj for Color {
    fn name(&self) -> &'static str {
        "Color"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Command for Color {
    fn get_return_types(&self) -> ReturnTypeSet {
        ReturnTypeSet::Color
    }
}

#[async_trait::async_trait]
impl Parsable for Color {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        Ok((slice.end(), ReturnType::Null))
    }
}

impl TreeWriter for Color {
    fn write_lisp(&self) -> String {
        let this = self.inner.read();

        if this.length > 0 {
            format!("(color \"{}\"${}$${})", this.color, this.pos, this.length)
        } else {
            format!("(color \"none\")")
        }
    }

    fn write_lint(&self, writer: &mut LintWriter) {
        let this = self.inner.read();
        if this.length > 0 {
            writer.write_up_to(this.pos);
            writer.write_as(LintColor::Color, this.length);
        }
    }

    fn write_javascript(&self, _indent: u8) -> String {
        let this = self.inner.read();
        if this.length > 0 {
            format!("color(\"{}\")", this.color)
        } else {
            format!("color(TODO())")
        }
    }
}

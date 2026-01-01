use std::any::Any;

use super::{
    Command, Context, FailReason, Indent, LintColor, LintWriter, Parsable, ParseTreeObj,
    ReturnType, ReturnTypeSet, RwLock, RwLockMappedWriteGuard, Slice, TreeWriter,
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
    fn get_child<'a>(&'a self, _index: usize) -> RwLockMappedWriteGuard<'a, dyn Command + 'static> {
        unreachable!()
    }
    fn len(&self) -> usize {
        0
    }
}

#[async_trait::async_trait]
impl Parsable for Color {
    async fn parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        if let Some((color, length)) = co.color_finder().find(slice) {
            let mut this = self.inner.write();
            *this = ColorData {
                color,
                pos: slice.pos,
                length,
            };

            Ok((slice.pos + length, ReturnType::Color))
        } else {
            Err(FailReason::NoLiteral)
        }
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

    fn write_lint(&self, writer: &mut LintWriter, _indent: u8) {
        let this = self.inner.read();
        if this.length > 0 {
            writer.write_up_to(this.pos);
            writer.write_as(LintColor::Color, this.length);
        }
    }

    fn write_javascript(&self, _indent: Indent) -> String {
        let this = self.inner.read();
        if this.length > 0 {
            format!("color(\"{}\")", this.color)
        } else {
            format!("color(TODO())")
        }
    }
}

use std::any::Any;
// use parking_lot::{Mutex, MutexGuard};

use super::{
    none::NoneCommand, AliasLoc, AliasName, Aliased, Command, Context, FailReason, Indent,
    LintWriter, Parsable, ParseTreeObj, ReturnType, ReturnTypeSet, RwLock, Slice, TreeWriter,
};

#[derive(Debug)]
pub struct StrokeData {
    pub child: Box<dyn Command>,
    pub optional: Option<[Box<dyn Command>; 2]>,
    pub loc: AliasLoc,
}

#[derive(Debug)]
pub struct Stroke {
    pub inner: RwLock<StrokeData>,
}

impl ParseTreeObj for Stroke {
    fn name(&self) -> &'static str {
        "Stroke"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Command for Stroke {
    fn get_return_types(&self) -> ReturnTypeSet {
        ReturnTypeSet::Void
    }
}

impl Aliased for Stroke {
    fn new(loc: AliasLoc) -> Self {
        Self {
            inner: RwLock::new(StrokeData {
                child: Box::new(NoneCommand::new()),
                optional: None,
                loc,
            }),
        }
    }

    fn alias() -> AliasName {
        *b"sto"
    }

    fn get_alias(&self) -> AliasName {
        Self::alias()
    }
}

#[async_trait::async_trait]
impl Parsable for Stroke {
    async fn try_parse(
        &self,
        _co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        // while let Some() = slice.{

        // }

        Ok((slice.end(), ReturnType::Void))
    }
}

// impl Expr for Stroke {

//     fn get_children(&self) -> RwLockReadGuard<'_, Vec<Box<dyn Stat>>> {
//        self.inner
//     }
// }

impl TreeWriter for Stroke {
    fn write_lisp(&self) -> String {
        let this = self.inner.read();

        let str = this.child.write_lisp();

        let str2 = this
            .optional
            .as_ref()
            .map_or(&[] as &[_], |e| e)
            .iter()
            .fold(str, |acc, ele| acc + " " + &ele.write_lisp());

        format!("(stroke{} {})", this.loc.write_lisp(), str2)
    }

    fn write_lint(&self, writer: &mut LintWriter, indent: u8) {
        let this = self.inner.read();
        this.loc.write_lint(writer, indent);
    }

    fn write_javascript(&self, indent: Indent) -> String {
        let this = self.inner.read();

        let str_first = this.child.write_javascript(indent.add());

        let str_chilren = this
            .optional
            .as_ref()
            .map_or(&[] as &[_], |e| e)
            .iter()
            .fold(str_first, |acc, ele| {
                acc + "," + &ele.write_javascript(indent.add())
            });

        let str = format!("{}set_stroke({})", indent.str(), str_chilren);

        str
    }
}

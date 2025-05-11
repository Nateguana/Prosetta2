use genawaiter::sync::Co;

use super::{
    commands::{Command, Parseable},
    parser_result::{ParserAction, ParserStep},
    slice::Slice,
    types::ReturnType,
    // slice::Slice,
    // types::ReturnType,
};

pub type Spot = Box<dyn Command>;

#[async_trait::async_trait]
pub trait Context: Send + Sync {
    async fn step_continue(&self, this: &dyn Command, pos: usize);
    // async fn step_paragraph(
    //     &self,
    //     this: &'static str,
    //     child: &mut Box<dyn Command>,
    //     slice: Slice<'_>,
    // ) -> Option<(usize, ReturnType)>;
    async fn step_child<T: Command + 'static>(
        &self,
        this: &dyn Parseable,
        child: &T,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)>;

    fn get_parent(&self) -> &dyn Parseable;
    // async fn step_match(&self, this: &'static str, child: &dyn Command, pos: usize);
    // async fn step_fail(&self, this: &'static str, child: &dyn Command, pos: usize);
}

// the context for the debug genenerator
pub struct DebugContextBase {
    co: Co<ParserStep>,
}

// pub struct RunContextBase {
// }

pub struct DebugContext<'a, 'b> {
    base: &'a DebugContextBase,
    inner: RunContext<'b>,
}

pub struct RunContext<'a> {
    parent: &'a dyn Parseable,
    level: u8,
}

impl DebugContextBase {
    pub fn new(co: Co<ParserStep>) -> Self {
        Self { co }
    }
}

impl<'a, 'b> DebugContext<'a, 'b> {
    pub fn new(base: &'a DebugContextBase, parent: &'b dyn Parseable) -> Self {
        Self {
            base,
            inner: RunContext::new(parent),
        }
    }
    pub fn new_from(&'a self, parent: &'b dyn Parseable) -> Self {
        Self {
            base: &self.base,
            inner: RunContext::new_from(&self.inner, parent),
        }
    }
}

impl<'a> RunContext<'a> {
    pub fn new(parent: &'a dyn Parseable) -> Self {
        Self { parent, level: 0 }
    }
    pub fn new_from(&self, parent: &'a dyn Parseable) -> Self {
        Self {
            parent,
            level: self.level + 1,
        }
    }
}

#[async_trait::async_trait]
impl<'a, 'b> Context for DebugContext<'a, 'b> {
    async fn step_continue(&self, this: &dyn Command, pos: usize) {
        self.base
            .co
            .yield_(ParserStep::new(
                ParserAction::Move { child: this.name() },
                pos,
            ))
            .await;
    }
    async fn step_child<T: Command + 'static>(
        &self,
        this: &dyn Parseable,
        child: &T,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)> {
        self.base
            .co
            .yield_(ParserStep::new(
                ParserAction::Child {
                    parent: this.name(),
                    child: child.name(),
                },
                slice.pos,
            ))
            .await;
        self.inner.step_child(this, child, slice).await;
        None
    }

    fn get_parent(&self) -> &'b dyn Parseable {
        self.inner.parent
    }
    // async fn step_paragraph(
    //     &self,
    //     this: &'static str,
    //     child: &mut Box<dyn Command>,
    //     slice: Slice<'_>,
    // ) -> Option<(usize, ReturnType)> {
    //     // let command = child.as_mut();
    //     // let parse_result = command.get_next_call(self, slice).await;
    //     // match parse_result {
    //     //     Ok(ret @ (pos, _)) => {
    //     //         self.step_match(this, command, pos).await;
    //     //         Some(ret)
    //     //     }
    //     //     Err(_fail_reason) => None,
    //     // }
    //     None
    // }
    // async fn step_child<T: Command + 'static>(
    //     &self,
    //     this: &dyn Command,
    //     spot: &mut Box<dyn Command>,
    //     slice: Slice<'_>,
    // ) -> Option<(usize, ReturnType)> {
    //     *spot = Box::new(T::new());
    //     self.step_paragraph(this.name(), spot, slice).await
    // }
}
#[async_trait::async_trait]
impl<'a> Context for RunContext<'a> {
    async fn step_continue(&self, _this: &dyn Command, _pos: usize) {}
    async fn step_child<T: Command + 'static>(
        &self,
        this: &dyn Parseable,
        child: &T,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)> {
        let context = RunContext::new_from(self, this);

        let ret = child.try_parse(context, slice).await;
        ret.ok()
    }

    fn get_parent(&self) -> &'a dyn Parseable {
        self.parent
    }
}

// async fn step_child<T: Command + 'static>(
//     co: impl Context,
//     child: &T,
//     slice: Slice<'_>,
// ) -> Option<(usize, ReturnType)> {
//     child.try_parse(co, slice).await
// }

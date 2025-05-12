use bitflags::parser;
use genawaiter::sync::Co;
use itertools::Position;

use super::{
    commands::{Command, Parseable},
    fail_reason::FailReason,
    parser_result::{self, ParserAction, ParserStep},
    slice::Slice,
    types::ReturnType, // slice::Slice,
                       // types::ReturnType,
};

const MAX_STACK_FRAME_LEVEL: u8 = 100;

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
    fn get_level(&self) -> u8;
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

        let context = self.new_from(this);

        let result = if self.inner.level < MAX_STACK_FRAME_LEVEL {
            child.try_parse(context, slice).await
        } else {
            Err(FailReason::StackFrameLimit)
        };

        let parser_step = match result {
            Ok((position, return_type)) => ParserStep::new(
                ParserAction::Matched {
                    parent: this.name(),
                    child: child.name(),
                    return_type
                },
                position,
            ),
            Err(reason) => ParserStep::new(
                ParserAction::Failed {
                    parent: this.name(),
                    child: child.name(),
                    reason,
                },
                slice.pos,
            ),
        };

        self.base.co.yield_(parser_step).await;

        result.ok()
    }

    fn get_parent(&self) -> &'b dyn Parseable {
        self.inner.parent
    }

    fn get_level(&self) -> u8 {
        self.inner.get_level()
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
        let context = self.new_from(this);

        if self.level < MAX_STACK_FRAME_LEVEL {
            let ret = child.try_parse(context, slice).await;
            ret.ok()
        } else {
            None
        }
    }

    fn get_parent(&self) -> &'a dyn Parseable {
        self.parent
    }

    fn get_level(&self) -> u8 {
        self.level
    }
}

// async fn step_child_impl<T: Command + 'static>(
//     co: impl Context,
//     child: &T,
//     slice: Slice<'_>,
// ) -> Result<(usize, ReturnType), FailReason> {
//     if co.get_level() <= MAX_STACK_FRAME_LEVEL {
//         child.try_parse(co, slice).await;
//     } else {
//         None
//     }
// }

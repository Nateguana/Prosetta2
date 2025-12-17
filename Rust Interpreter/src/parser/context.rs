use genawaiter::sync::Co;

use super::{
    color_finder::ColorFinder,
    commands::{Parsable, ParseTreeObj},
    fail_reason::FailReason,
    parser_result::{ParserAction, ParserStep},
    slice::Slice,
    types::ReturnType, // slice::Slice,
                       // types::ReturnType,
};

const MAX_STACK_FRAME_LEVEL: u8 = 100;

pub type Spot = Box<dyn Parsable>;

#[async_trait::async_trait]
pub trait Context: Send + Sync {
    async fn step_continue(&self, this: &dyn Parsable, pos: usize, description: String);
    // async fn step_paragraph(
    //     &self,
    //     this: &'static str,
    //     child: &mut Box<dyn Command>,
    //     slice: Slice<'_>,
    // ) -> Option<(usize, ReturnType)>;
    async fn step_child<T: Parsable + 'static>(
        &self,
        this: &dyn ParseTreeObj,
        child: &T,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)>;

    fn get_parent(&self) -> &dyn ParseTreeObj;
    fn get_level(&self) -> u8;
    fn is_debug(&self) -> bool;

    fn color_finder(&self) -> &ColorFinder;
    // async fn step_match(&self, this: &'static str, child: &dyn Command, pos: usize);
    // async fn step_fail(&self, this: &'static str, child: &dyn Command, pos: usize);
}

// the context for the debug genenerator
pub struct DebugContextBase {
    co: Co<ParserStep>,
    base: RunContextBase,
}

pub struct RunContextBase {
    color_finder: ColorFinder,
}

pub struct DebugContext<'a, 'b> {
    base: &'a DebugContextBase,
    inner: RunContext<'a, 'b>,
}

pub struct RunContext<'a, 'b> {
    base: &'a RunContextBase,
    parent: &'b dyn ParseTreeObj,
    level: u8,
}

impl DebugContextBase {
    pub fn new(co: Co<ParserStep>) -> Self {
        Self {
            co,
            base: RunContextBase::new(),
        }
    }
}

impl RunContextBase {
    pub fn new() -> Self {
        Self {
            color_finder: ColorFinder::new(),
        }
    }
}

impl<'a, 'b> DebugContext<'a, 'b> {
    pub fn new(base: &'a DebugContextBase, parent: &'b dyn ParseTreeObj) -> Self {
        Self {
            base,
            inner: RunContext::new(&base.base, parent),
        }
    }
    pub fn new_from(&'a self, parent: &'b dyn ParseTreeObj) -> Self {
        Self {
            base: &self.base,
            inner: RunContext::new_from(&self.inner, parent),
        }
    }
}

impl<'a, 'b> RunContext<'a, 'b> {
    pub fn new(base: &'a RunContextBase, parent: &'b dyn ParseTreeObj) -> Self {
        Self {
            base,
            parent,
            level: 0,
        }
    }
    pub fn new_from(&self, parent: &'b dyn ParseTreeObj) -> Self {
        Self {
            base: self.base,
            parent,
            level: self.level + 1,
        }
    }
}

#[async_trait::async_trait]
impl<'a, 'b> Context for DebugContext<'a, 'b> {
    async fn step_continue(&self, this: &dyn Parsable, pos: usize, description: String) {
        self.base
            .co
            .yield_(ParserStep::new(
                ParserAction::Continue {
                    child: this.get_name(),
                    description,
                },
                pos,
            ))
            .await;
    }
    async fn step_child<T: Parsable + 'static>(
        &self,
        this: &dyn ParseTreeObj,
        child: &T,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)> {
        self.base
            .co
            .yield_(ParserStep::new(
                ParserAction::Child {
                    parent: this.get_name(),
                    child: child.get_name(),
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
                    parent: this.get_name(),
                    child: child.get_name(),
                    return_type,
                },
                position,
            ),
            Err(reason) => ParserStep::new(
                ParserAction::Failed {
                    parent: this.get_name(),
                    child: child.get_name(),
                    reason,
                },
                slice.pos,
            ),
        };

        self.base.co.yield_(parser_step).await;

        result.ok()
    }

    fn get_parent(&self) -> &'b dyn ParseTreeObj {
        self.inner.parent
    }

    fn get_level(&self) -> u8 {
        self.inner.get_level()
    }
    fn is_debug(&self) -> bool {
        true
    }

    fn color_finder(&self) -> &ColorFinder {
        &self.base.base.color_finder
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
impl<'a, 'b> Context for RunContext<'a, 'b> {
    async fn step_continue(&self, _this: &dyn Parsable, _pos: usize, _description: String) {}
    async fn step_child<T: Parsable + 'static>(
        &self,
        this: &dyn ParseTreeObj,
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

    fn get_parent(&self) -> &'b dyn ParseTreeObj {
        self.parent
    }

    fn get_level(&self) -> u8 {
        self.level
    }

    fn is_debug(&self) -> bool {
        false
    }
    fn color_finder(&self) -> &ColorFinder {
        &self.base.color_finder
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

// #[macro_export]
macro_rules! Step_Continue {
    ($co:expr,$self:expr,$pos:expr,$format:expr) => {
        $co.step_continue(
            $self,
            $pos,
            format!($format, $self.get_name()),
        )
        .await;
    };
    ($co:expr,$self:expr,$pos:expr,$format:expr,$($args:expr),*) => {
        $co.step_continue(
            $self,
            $pos,
            format!($format, $self.get_name(), $($args:expr), *),
        )
        .await;
    };
}

pub(crate) use Step_Continue;

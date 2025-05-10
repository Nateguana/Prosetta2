use genawaiter::sync::Co;

use super::{
    commands::Command,
    parser_result::{ParserAction, ParserStep},
    slice::Slice,
    types::ReturnType,
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
        this: &dyn Command,
        spot: &mut Box<dyn Command>,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)>;
    // async fn step_match(&self, this: &'static str, child: &dyn Command, pos: usize);
    // async fn step_fail(&self, this: &'static str, child: &dyn Command, pos: usize);
}

// the context for the debug genenerator
pub struct DebugContextBase {
    co: Co<ParserStep>,
}

// pub struct RunContextBase {
// }

pub struct DebugContext {
    co: Co<ParserStep>,
    inner: RunContext,
    level: u8,
}

pub struct RunContext {
    co: Co<ParserStep>,
}

#[async_trait::async_trait]
impl Context for DebugContext {
    async fn step_continue(&self, this: &dyn Command, pos: usize) {
        self.co
            .yield_(ParserStep::new(
                ParserAction::Move { child: this.name() },
                pos,
            ))
            .await;
    }
    async fn step_paragraph(
        &self,
        this: &'static str,
        child: &mut Box<dyn Command>,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)> {
        // let command = child.as_mut();
        // let parse_result = command.get_next_call(self, slice).await;
        // match parse_result {
        //     Ok(ret @ (pos, _)) => {
        //         self.step_match(this, command, pos).await;
        //         Some(ret)
        //     }
        //     Err(_fail_reason) => None,
        // }
        None
    }
    async fn step_child<T: Command + 'static>(
        &self,
        this: &dyn Command,
        spot: &mut Box<dyn Command>,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)> {
        *spot = Box::new(T::new());
        self.step_paragraph(this.name(), spot, slice).await
    }
    async fn step_match(&self, this: &'static str, child: &dyn Command, pos: usize) {
        self.co
            .yield_(ParserStep::new(
                ParserAction::Matched {
                    parent: this,
                    child: child.name(),
                },
                pos,
            ))
            .await;
    }
    async fn step_fail(&self, this: &'static str, child: &dyn Command, pos: usize) {
        self.co
            .yield_(ParserStep::new(
                ParserAction::Failed {
                    parent: this,
                    child: child.name(),
                },
                pos,
            ))
            .await;
    }
}

impl DebugContext {
    pub fn new(co: Co<ParserStep>) -> Self {
        Self {
            co,
            inner: RunContext,
        }
    }
}

pub struct RunContext;

impl RunContext {
    pub fn new(co: Co<ParserStep>) -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Context for RunContext {
    async fn step_continue(&self, _this: &dyn Command, _pos: usize) {}
    async fn step_paragraph(
        &self,
        this: &'static str,
        child: &mut Box<dyn Command>,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)> {
        // let command = child.as_mut();
        // let parse_result = command.get_next_call(self, slice).await;
        // match parse_result {
        //     Ok(ret @ (pos, _)) => {
        //         self.step_match(this, command, pos).await;
        //         Some(ret)
        //     }
        //     Err(_fail_reason) => None,
        // }
        None
    }
    async fn step_child<T: Command + 'static>(
        &self,
        this: &dyn Command,
        spot: &mut Box<dyn Command>,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)> {
        *spot = Box::new(T::new());
        self.step_paragraph(this.name(), spot, slice).await
    }
}

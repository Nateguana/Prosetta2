use genawaiter::sync::Co;

use super::{
    commands::Command,
    parser_result::{ParserAction, ParserStep},
    slice::Slice,
    types::ReturnType,
};

pub type Spot = Box<dyn Command>;

pub struct Context {
    co: Co<ParserStep>,
}

impl Context {
    pub fn new(co: Co<ParserStep>) -> Self {
        Self { co }
    }
    pub async fn step_move(&self, this: &dyn Command, pos: usize) {
        self.co
            .yield_(ParserStep::new(
                ParserAction::Move { child: this.name() },
                pos,
            ))
            .await;
    }
    pub async fn step_spec_child(
        &self,
        this: &'static str,
        child: &mut Box<dyn Command>,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)> {
        let command = child.as_mut();
        let parse_result = command.try_parse(self, slice).await;
        match parse_result {
            Ok(ret @ (pos, _)) => {
                self.step_match(this, command, pos).await;
                Some(ret)
            }
            Err(_fail_reason) => None,
        }
    }
    pub async fn step_child<T: Command + 'static>(
        &self,
        this: &dyn Command,
        spot: &mut Box<dyn Command>,
        slice: Slice<'_>,
    ) -> Option<(usize, ReturnType)> {
        *spot = Box::new(T::new());
        self.step_spec_child(this.name(), spot, slice).await
    }
    pub async fn step_match(&self, this: &'static str, child: &dyn Command, pos: usize) {
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
    pub async fn step_fail(&self, this: &'static str, child: &dyn Command, pos: usize) {
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

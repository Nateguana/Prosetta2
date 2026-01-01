use std::any::Any;

use crate::parser::{parsable_vec::ParsableVec, ParserSource};

use super::{
    color_finder::ColorFinder,
    commands::{Parsable, ParseTreeObj},
    fail_reason::FailReason,
    slice::Slice,
    types::ReturnType, // slice::Slice,
                       // types::ReturnType,
};

pub type StepFunc<'a> = Box<dyn Fn(Context, Slice<'_>) -> ParseResult<'a>>;

// pub type ChildBackFunc<'a> = Box<dyn Fn(Context, Slice<'_>, Option<ReturnType>) -> ParseResult<'a>>;

pub enum ParseResult<'a> {
    Match {
        pos: usize,
        return_type: ReturnType,
    },
    Fail {
        reason: FailReason,
    },
    Continue {
        description: String,
        step: StepFunc<'a>,
        slice: Slice<'a>,
    },
    Child {
        child: usize,
        step: StepFunc<'a>,
        back: StepFunc<'a>,
        slice: Slice<'a>,
    },
}

pub struct ContextBase {
    pub debug: bool,
    pub level: u8,
    pub color_finder: ColorFinder,
    pub vec: ParsableVec,
    pub return_type: Option<ReturnType>,
    pub source: ParserSource,
}

impl ContextBase {
    pub fn make_step_method<'a, P: Parsable + 'static>(
        &'a mut self,
        step: impl Fn(&mut P, Context, Slice<'_>) -> ParseResult<'a>,
        index: usize,
    ) -> StepFunc<'a> {
        Box::new(move |co, slice| {
            step(
                self.vec
                    .get_mut(self.index)
                    .as_any()
                    .downcast_mut()
                    .unwrap(),
                co,
                slice,
            )
        })
    }
}

pub struct Context<'a> {
    base: &'a mut ContextBase,
    index: usize,
}

impl<'a> Context<'a> {
    pub fn new(base: &'a mut ContextBase, index: usize) -> Self {
        Self { base, index }
    }

    pub fn get_level(&self) -> u8 {
        self.base.level
    }
    pub fn is_debug(&self) -> bool {
        self.base.debug
    }
    pub fn color_finder(&self) -> &ColorFinder {
        &self.color_finder
    }

    pub fn get<T: Parsable + 'static>(&self, index: usize) -> Option<&T> {
        self.base.vec.get(index).as_any().downcast_ref::<T>()
    }
    pub fn get_mut<T: Parsable + 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.base.vec.get_mut(index).as_any().downcast_mut::<T>()
    }

    pub fn get_vec(&mut self) -> &mut ParsableVec {
        &mut self.vec
    }

    pub fn return_type(&self) -> Option<ReturnType> {
        self.base.return_type
    }

    pub fn into_root<T: Parsable + 'static>(self) -> Box<T> {
        Box::<dyn Any>::downcast(self.vec.into_root()).unwrap()
    }

    pub fn result_child<P: Parsable + 'static, C: Parsable + 'static>(
        &self,
        child: C,
        child_step: impl Fn(&mut C, Context, Slice<'_>) -> ParseResult<'a>,
        back_step: impl Fn(&mut P, Context, Slice<'_>, Option<ReturnType>) -> ParseResult<'a>,
        slice: Slice<'a>,
    ) -> (usize, ParseResult) {
        let index = self.base.vec.push(Box::new(child));

        let result = ParseResult::Child {
            child: index,
            step: Box::new(move |co, slice| {
                child_step(
                    self.base
                        .vec
                        .get_mut(index)
                        .as_any()
                        .downcast_mut()
                        .unwrap(),
                    co,
                    slice,
                )
            }),
            back: Box::new(move |co, slice| {
                back_step(
                    self.base
                        .vec
                        .get_mut(self.index)
                        .as_any()
                        .downcast_mut()
                        .unwrap(),
                    co,
                    slice,
                    co.return_type(),
                )
            }),
            slice,
        };

        (index, result)
    }

    pub fn result_cont<P: Parsable + 'static>(
        &self,
        step: impl Fn(&mut P, Context, Slice<'_>) -> ParseResult<'a>,
        slice: Slice<'a>,
        description: String,
    ) -> ParseResult {
        let result = ParseResult::Continue {
            step: self.base.make_step_method(step, self.index),
            description,
            slice,
        };

        result
    }
}

use std::any::Any;

use crate::parser::{
    parsable_vec::{ParsableVec, ParseableVecSplit},
    ParserSource,
};

use super::{
    color_finder::ColorFinder,
    commands::{Parsable, ParseTreeObj},
    fail_reason::FailReason,
    slice::Slice,
    types::ReturnType, // slice::Slice,
                       // types::ReturnType,
};

pub type StepFunc<'slice> = Box<
    dyn for<'cref, 'pref> Fn(
            &'cref mut ContextBase<'slice>,
            &'pref mut ParsableVec,
        ) -> ParseResult<'slice>
        + 'slice,
>;

pub enum ParseResult<'slice> {
    Match {
        pos: usize,
        return_type: ReturnType,
    },
    Fail {
        reason: FailReason,
    },
    Continue {
        description: String,
        step: StepFunc<'slice>,
        slice: Slice<'slice>,
    },
    Child {
        child: usize,
        step: StepFunc<'slice>,
        back: StepFunc<'slice>,
        slice: Slice<'slice>,
    },
}

pub struct ContextBase<'slice> {
    pub debug: bool,
    pub color_finder: ColorFinder,
    pub return_type: Option<(usize, ReturnType)>,
    pub source: &'slice mut ParserSource,
    pub max_stack_frame_level: u8,
}

impl<'slice> ContextBase<'slice> {
    pub fn make_root_method<P: Parsable + 'static>(
        step: impl for<'cref, 'pref> Fn(&'pref mut P, Context<'slice, 'cref, 'pref>) -> ParseResult<'slice>
            + 'static,
        index: usize,
    ) -> StepFunc<'slice> {
        Box::new(
            move |base: &mut ContextBase<'slice>, pvec: &mut ParsableVec| {
                let (this, vec_split) = pvec.split(index);
                step(
                    this.as_any_mut().downcast_mut().unwrap(),
                    Context::new(base, index, 1, Slice::empty(), vec_split),
                )
            },
        )
    }
}

pub struct Context<'slice:'cref, 'cref, 'pref> {
    base: &'cref mut ContextBase<'slice>,
    index: usize,
    slice: Slice<'slice>,
    level: u8,
    vec_split: ParseableVecSplit<'pref>,
}

impl<'slice:'cref, 'cref, 'pref> Context<'slice, 'cref, 'pref> {
    pub fn new(
        base: &'cref mut ContextBase<'slice>,
        index: usize,
        level: u8,
        slice: Slice<'slice>,
        vec_split: ParseableVecSplit<'pref>,
    ) -> Self {
        Self {
            base,
            index,
            level,
            slice,
            vec_split,
        }
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }
    pub fn is_debug(&self) -> bool {
        self.base.debug
    }
    pub fn color_finder(&self) -> &ColorFinder {
        &self.base.color_finder
    }

    pub fn get<T: Parsable + 'static>(&self, index: usize) -> Option<&T> {
        self.vec_split.get(index).as_any().downcast_ref::<T>()
    }
    pub fn get_mut<T: Parsable + 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.vec_split
            .get_mut(index)
            .as_any_mut()
            .downcast_mut::<T>()
    }

    pub fn get_vec(&mut self) -> &mut ParseableVecSplit<'pref> {
        &mut self.vec_split
    }

    pub fn get_source(&self) -> &'slice mut ParserSource {
        self.base.source
    }

    pub fn get_slice(&self) -> Slice {
        self.slice
    }


    fn make_child_step<C: Parsable + 'static>(
        &self,
        child_index: usize,
        child_step: impl for<'cref2> Fn(
                &'cref2 mut C,
                Context<'slice, 'cref2, 'cref2>,
                Slice<'slice>,
            ) -> ParseResult<'slice>
            + 'slice,
        slice: Slice<'slice>,
    ) -> StepFunc<'slice> {
        let new_level = self.level + 1;
        Box::new(move |base, pvec| {
            if new_level > base.max_stack_frame_level {
                ParseResult::Fail {
                    reason: FailReason::StackFrameLimit,
                }
            } else {
                let (this, vec_split) = pvec.split(child_index);
                child_step(
                    this.as_any_mut().downcast_mut().unwrap(),
                    Context::new(base, child_index, new_level, slice, vec_split),
                    slice,
                )
            }
        })
    }

    fn make_parent_step<P: Parsable + 'static>(
        &self,
        back_step: impl for<'cref2> Fn(
                &'cref2 mut P,
                Context<'slice, 'cref2, 'cref2>,
                Slice<'slice>,
                Option<ReturnType>,
            ) -> ParseResult<'slice>
            + 'slice,
        slice: Slice<'slice>,
    ) -> StepFunc<'slice> {
        let level = self.level;
        let this_index = self.index;
        Box::new(move |base, pvec| {
            let mut child_return = None;
            let mut slice = slice;
            if let Some((index, return_type)) = base.return_type {
                child_return = Some(return_type);
                slice = slice.start_at(index);
            }
            let (this, vec_split) = pvec.split(this_index);
            let co = Context::new(base, this_index, level, slice, vec_split);
            back_step(
                this.as_any_mut().downcast_mut::<P>().unwrap(),
                co,
                slice,
                child_return,
            )
        })
    }

    pub fn result_child<P: Parsable + 'static, C: Parsable + 'static>(
        mut self,
        child: C,
        child_step: impl for<'cref2> Fn(
                &'cref2 mut C,
                Context<'slice, 'cref2, 'cref2>,
                Slice<'slice>,
            ) -> ParseResult<'slice>
            + 'slice,
        back_step: impl for<'cref2> Fn(
                &'cref2 mut P,
                Context<'slice, 'cref2, 'cref2>,
                Slice<'slice>,
                Option<ReturnType>,
            ) -> ParseResult<'slice>
            + 'slice,
        slice: Slice<'slice>,
    ) -> (usize, ParseResult) {
        let child_index = self.vec_split.push(Box::new(child));
        // let parent_index = self.index;

        let result = ParseResult::Child {
            child: child_index,
            step: self.make_child_step(child_index, child_step, slice),
            back: self.make_parent_step(back_step, slice),
            slice,
        };

        (child_index, result)
    }

    pub fn result_cont<P: Parsable + 'static>(
        self,
        mut step: impl FnMut(&mut P, Context, Slice<'_>) -> ParseResult<'a> + 'static,
        slice: Slice<'a>,
        description: String,
    ) -> ParseResult<'a> {
        let result = ParseResult::Continue {
            step: Box::new(move |base| {
                step(
                    self.base
                        .vec
                        .get_mut(self.index)
                        .as_any()
                        .downcast_mut()
                        .unwrap(),
                    Context::new(base, self.index, self.level, slice),
                    slice,
                )
            }),
            description,
            slice,
        };

        result
    }

    pub fn result_match(self, pos: usize, return_type: ReturnType) -> ParseResult<'slice> {
        ParseResult::Match { pos, return_type }
    }

    pub fn result_fail(self, reason: FailReason) -> ParseResult<'slice> {
        ParseResult::Fail { reason }
    }
}

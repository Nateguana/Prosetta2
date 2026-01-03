use crate::parser::{
    parsable_vec::{ParsableVec, ParseableVecSplit},
    slice::SliceData,
    ParserSource,
};

use super::{
    color_finder::ColorFinder,
    commands::Parsable,
    fail_reason::FailReason,
    slice::Slice,
    types::ReturnType, // slice::Slice,
                       // types::ReturnType,
};

pub type StepFunc = Box<
    dyn for<'cref, 'pref> FnOnce(
            &'cref mut ContextBase,
            &'pref mut ParsableVec,
            &'cref mut ParserSource,
        ) -> ParseResult
        + 'static,
>;

pub enum ParseResult {
    Match {
        pos: usize,
        return_type: ReturnType,
    },
    Fail {
        reason: FailReason,
    },
    Continue {
        description: String,
        step: StepFunc,
        slice: SliceData,
    },
    Child {
        child: usize,
        step: StepFunc,
        back: StepFunc,
        slice: SliceData,
    },
}

pub struct ContextBase {
    pub debug: bool,
    pub color_finder: ColorFinder,
    pub return_type: Option<(usize, ReturnType)>,
    pub max_stack_frame_level: u8,
    pub source_index: usize,
}

impl ContextBase {
    pub fn make_root_method<'cref, P: Parsable + 'static>(
        step: impl for<'cref2, 'pref2> FnOnce(
                &'pref2 mut P,
                Context<'cref2, 'pref2>,
                &'cref2 mut ParserSource,
            ) -> ParseResult
            + 'static,
        index: usize,
    ) -> StepFunc {
        Box::new(
            move |base: &mut ContextBase, pvec: &mut ParsableVec, source: &mut ParserSource| {
                let (this, vec_split) = pvec.split(index);
                step(
                    this.as_any_mut().downcast_mut().unwrap(),
                    Context::new(base, index, 1, vec_split),
                    source,
                )
            },
        )
    }
}

pub struct Context<'cref, 'pref> {
    base: &'cref mut ContextBase,
    index: usize,
    level: u8,
    vec_split: ParseableVecSplit<'pref>,
}

impl<'cref, 'pref> Context<'cref, 'pref> {
    pub fn new(
        base: &'cref mut ContextBase,
        index: usize,
        level: u8,
        // slice: Slice<'slice>,
        vec_split: ParseableVecSplit<'pref>,
    ) -> Self {
        Self {
            base,
            index,
            level,
            // slice,
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

    pub fn set_source_index(&mut self, index: usize) {
        self.base.source_index = index;
    }
    pub fn get_source_index(&self) -> usize {
        self.base.source_index
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

    // pub fn get_slice(&self) -> Slice {
    //     self.slice
    // }

    fn make_child_step<C: Parsable + 'static>(
        &self,
        child_index: usize,
        child_step: impl for<'cref2, 'pref2> FnOnce(&'pref2 mut C, Context<'cref2, 'pref2>, Slice) -> ParseResult
            + 'static,
        slice_data: SliceData,
    ) -> StepFunc {
        let new_level = self.level + 1;
        Box::new(move |base, pvec, source| {
            let slice = Slice::from(source.get_source(base.source_index).unwrap(), slice_data);
            if new_level > base.max_stack_frame_level {
                ParseResult::Fail {
                    reason: FailReason::StackFrameLimit,
                }
            } else {
                let (this, vec_split) = pvec.split(child_index);
                child_step(
                    this.as_any_mut().downcast_mut().unwrap(),
                    Context::new(base, child_index, new_level, vec_split),
                    slice,
                )
            }
        })
    }

    fn make_parent_step<P: Parsable + 'static>(
        &self,
        back_step: impl for<'cref2, 'pref2> FnOnce(
                &'pref2 mut P,
                Context<'cref2, 'pref2>,
                Slice,
                Option<ReturnType>,
            ) -> ParseResult
            + 'static,
        slice_data: SliceData,
    ) -> StepFunc {
        let level = self.level;
        let this_index = self.index;
        Box::new(move |base, pvec, source| {
            let slice = Slice::from(source.get_source(base.source_index).unwrap(), slice_data);
            let mut child_return = None;
            let mut slice = slice;
            if let Some((index, return_type)) = base.return_type {
                child_return = Some(return_type);
                slice = slice.start_at(index);
            }
            let (this, vec_split) = pvec.split(this_index);
            let co = Context::new(base, this_index, level, vec_split);
            back_step(
                this.as_any_mut().downcast_mut::<P>().unwrap(),
                co,
                slice,
                child_return,
            )
        })
    }

    pub fn result_root_child<P: Parsable + 'static, C: Parsable + 'static>(
        mut self,
        child: C,
        child_step: impl for<'cref2, 'pref2> FnOnce(&'pref2 mut C, Context<'cref2, 'pref2>, Slice) -> ParseResult
            + 'static,
        back_step: impl for<'cref2, 'pref2> FnOnce(
                &'pref2 mut P,
                Context<'cref2, 'pref2>,
                &'cref2 mut ParserSource,
            ) -> ParseResult
            + 'static,
        slice: Slice,
    ) -> (usize, ParseResult) {
        let child_index = self.vec_split.push(Box::new(child));
        // let parent_index = self.index;
        let slice_data = slice.data();

        let result = ParseResult::Child {
            child: child_index,
            step: self.make_child_step(child_index, child_step, slice_data),
            back: ContextBase::make_root_method(back_step, self.index),
            slice: slice_data,
        };

        (child_index, result)
    }

    pub fn result_child<P: Parsable + 'static, C: Parsable + 'static>(
        mut self,
        child: C,
        child_step: impl for<'cref2, 'pref2> FnOnce(&'pref2 mut C, Context<'cref2, 'pref2>, Slice) -> ParseResult
            + 'static,
        back_step: impl for<'cref2, 'pref2> FnOnce(
                &'pref2 mut P,
                Context<'cref2, 'pref2>,
                Slice,
                Option<ReturnType>,
            ) -> ParseResult
            + 'static,
        slice: Slice,
    ) -> (usize, ParseResult) {
        let child_index = self.vec_split.push(Box::new(child));
        // let parent_index = self.index;
        let slice_data = slice.data();

        let result = ParseResult::Child {
            child: child_index,
            step: self.make_child_step(child_index, child_step, slice_data),
            back: self.make_parent_step(back_step, slice_data),
            slice: slice_data,
        };

        (child_index, result)
    }

    pub fn result_cont<P: Parsable + 'static>(
        self,
        step: impl for<'cref2, 'pref2> FnOnce(&'pref2 mut P, Context<'cref2, 'pref2>, Slice) -> ParseResult
            + 'static,
        slice: Slice,
        description: String,
    ) -> ParseResult {
        let slice_data = slice.data();
        let index = self.index;
        let level = self.level;

        let result = ParseResult::Continue {
            step: Box::new(move |base, pvec, source| {
                let slice = Slice::from(source.get_source(base.source_index).unwrap(), slice_data);
                let (this, vec_split) = pvec.split(index);
                let co = Context::new(base, index, level, vec_split);
                step(this.as_any_mut().downcast_mut::<P>().unwrap(), co, slice)
            }),
            description,
            slice: slice_data,
        };

        result
    }

    pub fn result_match(self, pos: usize, return_type: ReturnType) -> ParseResult {
        ParseResult::Match { pos, return_type }
    }

    pub fn result_fail(self, reason: FailReason) -> ParseResult {
        ParseResult::Fail { reason }
    }
}

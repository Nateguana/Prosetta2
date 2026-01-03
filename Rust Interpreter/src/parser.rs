#![allow(dead_code)]

// use std::{any::Any, future::IntoFuture, sync::Arc, task::Poll};

use std::{mem::ManuallyDrop, ptr::NonNull};

use context::Context;

// use parking_lot::{MappedMutexGuard, Mutex, MutexGuard, RwLock};
// pub(crate) use parser_result::{ParserAction, ParserData, ParserStep};
// use smol::lock::{RwLock, RwLockReadGuardArc};
// use alias::WordTriggerArena;
// use bstr::ByteSlice;
mod alias_data;
mod alias_finder;
mod child_vec;
mod close_data;
mod color_finder;
mod commands;
mod context;
mod fail_reason;
mod import_finder;
mod imports;
mod parsable_vec;
mod parser_result;
mod parser_tree_root;
mod rwlock;
mod slice;
mod source;
mod types;

use rwlock::{ArcRwLock, RwLockReadGuard};

pub(crate) mod tree_writer;

use commands::{paragraph_start, title::Title, Paragraph};
use slice::Slice;
pub use source::ParserSource;

use source::ParserSourceStepper;
use tokio::runtime::Builder;

use crate::parser::{
    color_finder::ColorFinder,
    context::{ContextBase, StepFunc},
    fail_reason::FailReason,
    parsable_vec::{ParsableVec, ParseableVecSplit},
    parser_tree_root::ParserTreeRoot,
    slice::SliceData,
    types::ReturnType,
};

// use source::ParserSourceIter;

// pub struct Paragraph {
//     // global_pos: usize,
//     data: ParagraphType,
// }

// pub enum ParagraphType {
//     Title(Box<Title>),
//     Regular(Vec<Box<dyn Command>>),
// }

pub struct Parser {
    source: ParserSource,
}

// #[derive(Debug)]
pub struct ParserData {
    pub source: ParserSource,
    pub tree: ParsableVec,
}

pub struct ParserDebug {
    context_base: ContextBase,
    stack: Vec<StepFunc>,
    parseables: ParsableVec,
    source: ParserSource,
}

const MAX_STACK_FRAME_LEVEL: u8 = 100;

impl Parser {
    ///make a new parser with a source
    pub fn new(source: ParserSource) -> Self {
        Self { source }
    }

    fn make_context(debug: bool) -> ContextBase {
        ContextBase {
            debug,
            color_finder: ColorFinder::new(),
            return_type: None,
            max_stack_frame_level: MAX_STACK_FRAME_LEVEL,
            source_index: 0,
        }
    }

    ///run the parser
    pub fn run(self) -> ParserData {
        let mut source = self.source;

        let parseables = Self::run_impl(&mut source);

        ParserData {
            source: source,
            tree: parseables,
        }
    }

    fn run_impl(source: &mut ParserSource) -> ParsableVec {
        let mut context_base = Self::make_context(false);

        let mut parseables = ParsableVec::new();

        parseables.push(Box::new(ParserTreeRoot::new()));

        let mut stack: Vec<StepFunc> = vec![ContextBase::make_root_method(
            move |this, co, source| ParserTreeRoot::parse(this, co, source),
            1,
        )];

        while let Some(func) = stack.pop() {
            //  println!("step");
            match func(&mut context_base, &mut parseables, source) {
                context::ParseResult::Match {
                    pos, return_type, ..
                } => {
                    context_base.return_type = Some((pos, return_type));
                }
                context::ParseResult::Fail { .. } => {}
                context::ParseResult::Continue { step, .. } => {
                    stack.push(step);
                }
                context::ParseResult::Child { step, back, .. } => {
                    stack.push(back);
                    stack.push(step);
                }
            }
            parseables.update();
        }

        // // SAFETY:
        // unsafe {
        //     ParserData {
        //         source: ManuallyDrop::take(&mut source),
        //         tree: parseables,
        //     }
        // }
        parseables
    }

    pub fn debug(self) -> ParserDebug {
        let mut parseables = ParsableVec::new();

        parseables.push(Box::new(ParserTreeRoot::new()));
        ParserDebug {
            context_base: Self::make_context(true),
            stack: vec![ContextBase::make_root_method(
                move |this, co, source| ParserTreeRoot::parse(this, co, source),
                1,
            )],
            parseables,
            source: self.source,
        }
    }
}

pub enum ParserStepResult {
    Done,
    Match {
        name: &'static str,
        pos: usize,
        return_type: ReturnType,
    },
    Fail {
        name: &'static str,
        reason: FailReason,
    },
    Continue {
        name: &'static str,
        description: String,
        slice: SliceData,
    },
    Child {
        child_name: &'static str,
        parent_name: &'static str,
        slice: SliceData,
    },
}

impl ParserDebug {
    pub fn step<'a>(&'a mut self) -> ParserStepResult {
        if let Some(func) = self.stack.pop() {
            //  println!("step");
            let result = match func(
                &mut self.context_base,
                &mut self.parseables,
                &mut self.source,
            ) {
                context::ParseResult::Match {
                    pos,
                    return_type,
                    name,
                    ..
                } => {
                    self.context_base.return_type = Some((pos, return_type));
                    ParserStepResult::Match {
                        pos,
                        return_type,
                        name,
                    }
                }
                context::ParseResult::Fail { reason, name } => {
                    ParserStepResult::Fail { name, reason }
                }
                context::ParseResult::Continue {
                    step,
                    slice,
                    name,
                    description,
                } => {
                    self.stack.push(step);
                    ParserStepResult::Continue {
                        name,
                        description,
                        slice,
                    }
                }
                context::ParseResult::Child {
                    step,
                    back,
                    child_name,
                    parent_name,
                    slice,
                    ..
                } => {
                    self.stack.push(back);
                    self.stack.push(step);
                    ParserStepResult::Child {
                        child_name,
                        parent_name,
                        slice,
                    }
                }
            };
            self.parseables.update();
            result
        } else {
            ParserStepResult::Done
        }
    }
    ///step the parser
    pub fn tree<'a>(&'a self) -> &'a ParsableVec {
        &self.parseables
    }

    pub fn get_source<'a>(&'a self) -> &'a ParserSource {
        &self.source
    }

    pub fn into_data(self) -> ParserData {
        ParserData {
            source: self.source,
            tree: self.parseables,
        }
    }
}

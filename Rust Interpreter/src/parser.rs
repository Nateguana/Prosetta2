#![allow(dead_code)]

// use std::{any::Any, future::IntoFuture, sync::Arc, task::Poll};

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

// pub struct ParserDebug<'a> {
//     context_base: ContextBase,
//     stack: Vec<StepFunc<'a>>,
// }

const MAX_STACK_FRAME_LEVEL: u8 = 100;

// enum NextStepFunc<T> {
//     Step(StepContinueFunc<T>),
//     Child(ChildContinueFunc<T>, Option<(usize, ReturnType)>),
// }

// type NextStepFunc = Box<dyn Fn()>

impl Parser {
    ///make a new parser with a source
    pub fn new(source: ParserSource) -> Self {
        Self { source }
    }

    fn make_context(debug: bool, source: &mut ParserSource) -> ContextBase {
        ContextBase {
            debug,
            color_finder: ColorFinder::new(),
            return_type: None,
            max_stack_frame_level: MAX_STACK_FRAME_LEVEL,
        }
    }

    pub fn run(self) -> ParserData {
        let mut source = self.source;
        let parseables = self.run_impl(&mut source);
        ParserData {
            source: source,
            tree: parseables,
        }
    }

    fn run_impl(self, source: &mut ParserSource) -> ParsableVec {
        let mut context_base = Self::make_context(false);

        let mut parseables = ParsableVec::new();

        parseables.push(Box::new(ParserTreeRoot::new()));

        let mut stack: Vec<StepFunc<'_>> =
            vec![ContextBase::make_root_method(ParserTreeRoot::parse, 1)];

        while let Some(func) = stack.pop() {
            match func(&mut context_base, &mut parseables) {
                context::ParseResult::Match { pos, return_type } => {
                    context_base.return_type = Some((pos, return_type));
                }
                context::ParseResult::Fail { reason: _ } => {}
                context::ParseResult::Continue {
                    description: _,
                    step,
                    slice: _,
                } => {
                    stack.push(step);
                }
                context::ParseResult::Child {
                    child: _,
                    step,
                    back,
                    slice: _,
                } => {
                    stack.push(back);
                    stack.push(step);
                }
            }
            parseables.update();
        }

        parseables
    }

    // pub fn debug(self) -> ParserDebug {
    //     ParserDebug {
    //         context_base: self.make_context(false),
    //         stack: Vec::new(),
    //     }
    // }
}

// enum ParserStepResult {
//     Done,
//     Match {
//         pos: usize,
//         return_type: ReturnType,
//     },
//     Fail {
//         reason: FailReason,
//     },
//     Continue {
//         description: String,
//         step: StepFunc,
//         slice: Slice,
//     },
//     Child {
//         child: usize,
//         step: StepFunc,
//         back: StepFunc,
//         slice: Slice,
//     },
// }

// impl ParserDebug {
//     pub fn step(&mut self) -> ParserStepResult {
//         if let Some(func) = stack.pop() {
//             match func(&mut context_base) {
//                 context::ParseResult::Match { pos, return_type } => {
//                     context_base.return_type = Some((pos, return_type));
//                     ParserStepResult
//                 }
//                 context::ParseResult::Fail { reason } => {

//                 },
//                 context::ParseResult::Continue {
//                     description,
//                     step,
//                     slice,
//                 } => {
//                     stack.push(step);
//                 }
//                 context::ParseResult::Child {
//                     child,
//                     step,
//                     back,
//                     slice,
//                 } => {
//                     stack.push(back);
//                     stack.push(step);
//                 }
//             }
//         } else {
//             ParserStepResult::Done
//         }
//     }
// }

// impl ParserDebug {
//     ///step the parser
//     pub fn step(&mut self) -> ParserStep {
//         match self.generator.resume() {
//             genawaiter::GeneratorState::Yielded(step) => step,
//             genawaiter::GeneratorState::Complete(step) => {
//                 self.is_generator_done = true;
//                 step
//             }
//         }
//     }

//     pub fn next(&mut self) -> Option<ParserStep> {
//         if self.is_generator_done {
//             None
//         } else {
//             Some(self.step())
//         }
//     }

//     pub fn tree<'a>(&'a self) -> RwLockReadGuard<'a, Vec<Box<dyn Paragraph>>> {
//         self.parser.tree.read()
//     }

//     pub fn get_source<'a>(&'a self) -> RwLockReadGuard<'a, ParserSource> {
//         self.parser.source.read()
//     }

//     pub fn into_data(self) -> ParserData {
//         ParserData {
//             source: self.parser.source.into_inner(),
//             tree: self.parser.tree.into_inner(),
//         }
//     }
// }

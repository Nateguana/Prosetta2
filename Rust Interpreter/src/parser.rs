#![allow(dead_code)]

// use std::{any::Any, future::IntoFuture, sync::Arc, task::Poll};

use context::Context;

// use parking_lot::{MappedMutexGuard, Mutex, MutexGuard, RwLock};
pub(crate) use parser_result::{ParserAction, ParserData, ParserStep};
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
    context::{ContextBase, StepContinueFunc, StepFunc},
    parsable_vec::ParsableVec,
    parser_tree_root::ParserTreeRoot,
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

// pub struct ParserDebug {
//     parser: Parser,
//     is_generator_done: bool,
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

    pub fn run(self) -> ParserData {
        let mut context_base = ContextBase {
            debug: false,
            level: 0,
            color_finder: ColorFinder::new(),
            vec: ParsableVec::new(),
            return_type: None,
            source: self.source,
        };

        context_base.vec.push(Box::new(ParserTreeRoot::new()));
        let mut stack: Vec<StepFunc> = vec![context_base.make_step_method(ParserTreeRoot::parse, 1)];

        while let Some(func) = stack.pop() {
            let context = Context::new(&mut context_base)

            // match func()
        }

        ParserData {
            source: context_base.source,
            tree: context_base.vec,
        }
    }

    // pub fn debug(self) -> ParserDebug {
    //     ParserDebug {
    //         parser: self,
    //         is_generator_done: false,
    //     }
    // }
}

// fn step_paragraph(
//     co: impl Context,
//     source: ParserSource,
//     tree: Vec<Box<dyn Paragraph>>,
// ) -> Box<dyn Fn(&mut T) -> ParseResult<T>> {
//     let mut has_title = false;

//     let mut parser_stepper = ParserSourceStepper::new();

//     let mut paragraph_index = 0;
//     loop {
//         parser_stepper.step(&mut source);
//         if let Some(paragraph) = parser_stepper.next(&mut source) {
//             let slice = Slice::new(&*paragraph);

//             if !has_title {
//                 let title = Box::new(Title::new(paragraph_index)) as Box<dyn Paragraph>;
//                 tree.push(title);

//                 let title_ref = tree
//                     .last()
//                     .unwrap()
//                     .as_any()
//                     .downcast_ref::<Title>()
//                     .unwrap();

//                 co.step_child(co.get_parent(), title_ref, slice);
//                 has_title = true;
//             } else {
//             }

//             paragraph_index += 1;
//         } else {
//             break;
//         }
//     }
// }

// impl ParserDebug {
//     async fn start_debug(
//         co: Co<ParserStep>,
//         source: ArcRwLock<ParserSource>,
//         tree: ArcRwLock<Vec<Box<dyn Paragraph>>>,
//     ) -> ParserStep {
//         let global_parent = commands::none::NoneStart::new();
//         let mut context_base = DebugContextBase::new(co);
//         let context = DebugContext::new(&mut context_base, &global_parent);
//         Parser::start(context, source, tree).await;

//         let finish_action = ParserAction::Finished;

//         ParserStep::new(finish_action, 0)
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

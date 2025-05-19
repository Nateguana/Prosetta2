#![allow(dead_code)]

// use std::{any::Any, future::IntoFuture, sync::Arc, task::Poll};

use context::{Context, DebugContext, DebugContextBase, RunContext};
use genawaiter::sync::{Co, GenBoxed};

// use parking_lot::{MappedMutexGuard, Mutex, MutexGuard, RwLock};
pub(crate) use parser_result::{ParserAction, ParserData, ParserStep};
// use smol::lock::{RwLock, RwLockReadGuardArc};
// use alias::WordTriggerArena;
// use bstr::ByteSlice;
mod alias_finder;
mod close_data;
mod commands;
mod context;
mod fail_reason;
mod imports;
mod parser_result;
mod rwlock;
mod slice;
mod source;
mod types;

use rwlock::{ArcRwLock, RwLockReadGuard};

pub(crate) mod javascript_writer;
pub(crate) mod lisp_like_writer;
pub(crate) mod syntax_writer;

use commands::{paragraph_start, title::Title, Paragraph};
use slice::Slice;
pub use source::ParserSource;

use source::ParserSourceStepper;
use tokio::runtime::Builder;
// use source::ParserSourceIter;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// pub struct Paragraph {
//     // global_pos: usize,
//     data: ParagraphType,
// }

// pub enum ParagraphType {
//     Title(Box<Title>),
//     Regular(Vec<Box<dyn Command>>),
// }

pub struct Parser {
    source: ArcRwLock<ParserSource>,
    tree: ArcRwLock<Vec<Box<dyn Paragraph>>>,
}

pub struct ParserDebug {
    parser: Parser,
    generator: GenBoxed<ParserStep, (), ParserStep>,
    is_generator_done: bool,
}

impl Parser {
    ///make a new parser with a source and command flags
    pub fn new(source: ParserSource) -> Self {
        Self {
            source: ArcRwLock::new(source),
            tree: ArcRwLock::new(Vec::new()),
        }
    }

    pub fn run(self) -> ParserData {
        let global_parent = commands::none::NoneStart::new();
        // let mut context_base = RunContextBase::new();
        let context = RunContext::new(&global_parent);

        let result = Parser::start(context, self.source.clone(), self.tree.clone());
        // let res2 = result.into_future();
        let rt = Builder::new_current_thread().build().unwrap();
        rt.block_on(result);
        ParserData {
            source: self.source.into_inner(),
            tree: self.tree.into_inner(),
        }
    }

    pub fn debug(self) -> ParserDebug {
        let generator = GenBoxed::new_boxed(|co| {
            ParserDebug::start_debug(co, self.source.clone(), self.tree.clone())
        });

        ParserDebug {
            parser: self,
            generator,
            is_generator_done: false,
        }
    }

    async fn start(
        co: impl Context,
        source: ArcRwLock<ParserSource>,
        tree: ArcRwLock<Vec<Box<dyn Paragraph>>>,
    ) {
        let mut has_title = false;
        // let mut iter = source.get_mut_iter();

        let mut parser_stepper = ParserSourceStepper::new();

        let paragraph_index = 0;
        loop {
            parser_stepper.step(&mut source.write());
            if let Some(paragraph) = parser_stepper.next(&source.read()) {
                let slice = Slice::new(&*paragraph);
                if !has_title {
                    {
                        let mut tree_lock = tree.write();
                        let title = Box::new(Title::new(paragraph_index)) as Box<dyn Paragraph>;
                        tree_lock.push(title);
                    }
                    let title_lock = tree.read();
                    let title_ref = title_lock
                        .last()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Title>()
                        .unwrap();

                    co.step_child(co.get_parent(), title_ref, slice).await;
                    has_title = true;
                }
            } else {
                break;
            }
        }
    }
}

impl ParserDebug {
    async fn start_debug(
        co: Co<ParserStep>,
        source: ArcRwLock<ParserSource>,
        tree: ArcRwLock<Vec<Box<dyn Paragraph>>>,
    ) -> ParserStep {
        let global_parent = commands::none::NoneStart::new();
        let mut context_base = DebugContextBase::new(co);
        let context = DebugContext::new(&mut context_base, &global_parent);
        Parser::start(context, source, tree).await;

        let finish_action = ParserAction::Finished;

        ParserStep::new(finish_action, 0)
    }
}

impl ParserDebug {
    ///step the parser
    pub fn step(&mut self) -> ParserStep {
        match self.generator.resume() {
            genawaiter::GeneratorState::Yielded(step) => step,
            genawaiter::GeneratorState::Complete(step) => {
                self.is_generator_done = true;
                step
            }
        }
    }

    pub fn next(&mut self) -> Option<ParserStep> {
        if self.is_generator_done {
            None
        } else {
            Some(self.step())
        }
    }

    pub fn tree(&self) -> RwLockReadGuard<Vec<Box<dyn Paragraph>>> {
        self.parser.tree.read()
    }

    pub fn get_source(&self) -> RwLockReadGuard<ParserSource> {
        self.parser.source.read()
    }

    pub fn into_data(self) -> ParserData {
        ParserData {
            source: self.parser.source.into_inner(),
            tree: self.parser.tree.into_inner(),
        }
    }
}

// struct ParserIter<'a> {
//     parser: &'a mut Parser,
// }

// impl<'a> Iterator for ParserIter<'a> {
//     type Item = ParserStep;

//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.parser.is_generator_done {
//             Some(self.parser.step())
//         } else {
//             None
//         }
//     }
// }

#![allow(dead_code)]

// use std::{any::Any, future::IntoFuture, sync::Arc, task::Poll};

use context::{Context, DebugContext, DebugContextBase, RunContext};
use genawaiter::sync::{Co, GenBoxed};

// use parking_lot::{MappedMutexGuard, Mutex, MutexGuard, RwLock};
pub(crate) use parser_result::{ParserAction, ParserData, ParserStep};
// use smol::lock::{RwLock, RwLockReadGuardArc};
// use alias::WordTriggerArena;
// use bstr::ByteSlice;
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

use rwlock::{ArcRwLock, RwLockReadGuardArc};

pub(crate) mod javascript_writer;
pub(crate) mod lisp_like_writer;

use commands::{title::Title, Paragraph, Parseable};
use slice::Slice;
pub use source::ParserSource;

use source::ParserSourceStepper;
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
    generator: GenBoxed<ParserStep, (), ParserStep>,
    tree: ArcRwLock<Vec<Box<dyn Paragraph>>>,
    is_generator_done: bool,
}

impl Parser {
    ///make a new parser with a source and command flags
    pub fn new(source: ParserSource) -> Parser {
        let arc_source = ArcRwLock::new(source);
        let lock_tree = ArcRwLock::new(Vec::new());
        let generator = GenBoxed::new_boxed(|co| {
            Parser::start_debug(co, arc_source.clone(), lock_tree.clone())
        });

        Parser {
            source: arc_source,
            generator,
            tree: lock_tree,
            is_generator_done: false,
        }
    }

    pub fn run(source: ParserSource) -> ParserData {
        let arc_source = ArcRwLock::new(source);
        let tree = ArcRwLock::new(Vec::new());

        let global_parent = commands::none::NoneStart::new();
        // let mut context_base = RunContextBase::new();
        let context = RunContext::new(&global_parent);

        let result = Parser::start(context, arc_source.clone(), tree.clone());
        // let res2 = result.into_future();
        smol::block_on(result);
        ParserData {
            source: arc_source.into_inner(),
            tree: tree.into_inner(),
        }
    }

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

    async fn start(
        co: impl Context,
        source: ArcRwLock<ParserSource>,
        tree: ArcRwLock<Vec<Box<dyn Paragraph>>>,
    ) {
        let has_title = false;
        // let mut iter = source.get_mut_iter();

        let mut parser_stepper = ParserSourceStepper::new();

        loop {
            parser_stepper.step(&mut source.write());
            if let Some(paragraph) = parser_stepper.next(&source.read()) {
                let slice = Slice::new(&*paragraph);
                if !has_title {
                    {
                        let mut tree_lock = tree.write();
                        let title = Box::new(Title::new()) as Box<dyn Paragraph>;
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
                }
            } else {
                break;
            }
        }
    }
}

///the parser - Woah!!
impl Parser {
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

    pub fn tree(&self) -> RwLockReadGuardArc<Vec<Box<dyn Paragraph>>> {
        self.tree.read()
    }

    pub fn get_source(&self) -> RwLockReadGuardArc<ParserSource> {
        self.source.read()
    }

    pub fn into_data(self) -> ParserData {
        ParserData {
            source: self.source.into_inner(),
            tree: self.tree.into_inner(),
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

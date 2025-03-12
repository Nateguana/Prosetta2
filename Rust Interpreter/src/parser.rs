#![allow(dead_code)]

use std::sync::Arc;

use context::Context;
use genawaiter::sync::{Co, GenBoxed};

use parking_lot::RwLock;
pub(crate) use parser_result::{ParserAction, ParserData, ParserStep};
// use alias::WordTriggerArena;
// use bstr::ByteSlice;
mod close_data;
mod commands;
mod context;
mod fail_reason;
mod imports;
mod parser_result;
mod slice;
mod source;
mod types;

pub(crate) mod javascript_writer;
pub(crate) mod lisp_like_writer;

use commands::{title::Title, Command};
use slice::Slice;
pub use source::ParserSource;

use source::ParserSourceIter;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub struct Paragraph {
    // global_pos: usize,
    data: ParagraphType,
}

pub enum ParagraphType {
    Title(Box<Title>),
    Regular(Vec<Box<dyn Command>>),
}

pub struct Parser {
    source: Arc<ParserSource>,
    generator: GenBoxed<ParserStep, (), ParserStep>,
    tree: RwLock<Vec<Paragraph>>,
    is_generator_done: bool,
}

impl Parser {
    ///make a new parser with a source and command flags
    pub fn new(source: ParserSource) -> Parser {
        let arc_source = Arc::new(source);
        let lock_tree = RwLock::new(Vec::new());
        let generator = GenBoxed::new_boxed(|co| Parser::start(co, arc_source.clone(), lock_tree));

        Parser {
            source: arc_source,
            generator,
            tree: lock_tree,
            is_generator_done: false,
        }
    }

    pub async fn start(
        co: Co<ParserStep>,
        source: Arc<ParserSource>,
        tree: RwLock<Vec<Paragraph>>,
    ) -> ParserStep {
        let context = Context::new(co);

        let has_title = false;
        // let mut iter = source.get_mut_iter();

        for paragraph in source.get_mut_iter() {
            let slice = Slice::new(&*paragraph);
            if !has_title {
                let mut title = Box::new(Title::new()) as Box<dyn Command>;
                self.tree.push(Paragraph {
                    global_pos: 0,
                    data: ParagraphType::Title(title),
                });

                context.step_spec_child("Base", &mut title, slice).await;
            }
        }

        let finish_action = ParserAction::Finished {
            data: Box::new(ParserData {}),
        };

        return ParserStep::new(finish_action, 0);
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

    pub fn tree(&self) -> &Vec<Paragraph> {
        &self.tree
    }

    // pub fn iter<'a>(&'a mut self) -> ParserIter<'a> {
    //     ParserIterator { parser: self }
    // }
    pub fn source_iter<'a>(&'a self) -> ParserSourceIter<'a> {
        self.source.get_iter()
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

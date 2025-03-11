#![allow(dead_code)]

use context::Context;
use genawaiter::sync::{Co, GenBoxed};

pub(crate) use parser_result::{ParserAction, ParserData, ParserStep};
// use alias::WordTriggerArena;
// use bstr::ByteSlice;
mod close_data;
mod commands;
mod context;
mod fail_reason;
mod imports;
mod javascript_writer;
mod parser_result;
mod slice;
mod source;
mod types;

use commands::{none::Base, title::Title, Command};
use slice::Slice;
pub use source::ParserSource;

use streaming_iterator::StreamingIterator;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

struct Paragraph {
    global_pos: usize,
    data: ParagraphType,
}

enum ParagraphType {
    Title(Box<Title>),
    Regular(Vec<Box<dyn Command>>),
}

pub struct Parser {
    generator: GenBoxed<ParserStep, (), ParserStep>,
    tree: Vec<Paragraph>,
    is_generator_done: bool,
}

impl Parser {
    ///make a new parser with a source and command flags
    pub fn new(source: ParserSource) -> Parser {
        let generator = GenBoxed::new_boxed(|co| Parser::start(co, source));

        Parser {
            generator,
            tree: Vec::new(),
            is_generator_done: false,
        }
    }
    pub async fn start(co: Co<ParserStep>, mut source: ParserSource) -> ParserStep {
        let context = Context::new(co);
        let base = Base::new();

        let has_title = false;
        let mut iter = source.get_mut_iter();

        while let Some(paragraph) = iter.next() {
            let slice = Slice::new(paragraph);
            if !has_title {
                let mut title = Box::new(Title::new()) as Box<dyn Command>;
                context.step_spec_child(&base, &mut title, slice).await;
            }
        }

        let finish_action = ParserAction::Finished {
            data: Box::new(ParserData { source }),
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
}

impl Iterator for Parser {
    type Item = ParserStep;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_generator_done {
            Some(self.step())
        } else {
            None
        }
    }
}

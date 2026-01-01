use itertools::Itertools;

use crate::parser::context::{ChildType, ParsableVec};

use super::{
    commands::{title::Title, Parsable, ParseTreeObj},
    context::{Context, ParseResult},
    slice::Slice,
    source::ParserSourceStepper,
    tree_writer::{indent::Indent, lint_writer::LintWriter, TreeWriter},
    ParserSource,
};
use std::{any::Any, rc::Weak};

#[derive(Debug)]
pub struct ParserTreeRoot {
    pub tree: Vec<usize>,
}

impl ParseTreeObj for ParserTreeRoot {
    fn name(&self) -> &'static str {
        "Parser Tree Root"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Parsable for ParserTreeRoot {
    fn parse(&mut self, co: Context, _slice: Slice<'_>) -> ParseResult<Self> {
        self.step(co, ParserSourceStepper::new(), false)
    }

    fn get_children(&self) -> Vec<usize> {
        self.tree
    }
}

impl ParserTreeRoot {
    pub fn new(source: Weak<ParserSource>) -> ParserTreeRoot {
        Self {
            tree: Vec::new(),
        }
    }

    fn step(
        &mut self,
        co: Context,
        parser_stepper: ParserSourceStepper,
        has_title: bool,
    ) -> ParseResult<Self> {
        let paragraph_index = self.parser_stepper.step(&mut self.source);
        if let Some(paragraph) = parser_stepper.next(&mut self.source) {
            let slice = Slice::new(&*paragraph);

            let index;
            if !has_title {
                index = co.new_child(Title::new(paragraph_index));
                self.tree.push(index);
                has_title = true;
            } else {
                todo!()
            }

            ParseResult::Child {
                child: ChildType::Command(index),
                slice,
                step: Box::new(move |s: &mut Self, co, _, _| s.step(co, parser_stepper, has_title)),
            }
        } else {
            ParseResult::Match {
                pos: paragraph_index,
            }
        }
    }
}

impl TreeWriter for ParserTreeRoot {
    fn write_lisp(&self, vec: ParsableVec) -> String {
        self.tree
            .into_iter()
            .map(|index| vec.get(index).write_lisp(vec))
            .join("\n\n")
    }

    fn write_lint(&self, vec: ParsableVec, writer: &mut LintWriter, indent: u8) {
        for index in self.tree {
            vec.get(index).write_lint(vec, writer, indent);
        }
    }

    fn write_javascript(&self, vec: ParsableVec, indent: Indent) -> String {
        self.tree
            .into_iter()
            .map(|index| vec.get(index).write_javascript(vec, indent))
            .join("\n\n")
    }
}

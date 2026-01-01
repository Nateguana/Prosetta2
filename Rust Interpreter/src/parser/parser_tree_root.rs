use itertools::Itertools;

use crate::parser::{parsable_vec::ParsableVec, types::ReturnType};

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
    fn get_children(&self) -> Vec<usize> {
        self.tree
    }
}

impl ParserTreeRoot {
    pub fn new() -> ParserTreeRoot {
        Self { tree: Vec::new() }
    }

    pub fn parse(
        &mut self,
        co: Context,
        _slice: Slice<'_>,
        source: Weak<ParserSource>,
    ) -> ParseResult {
        self.step(co, source, ParserSourceStepper::new(), false)
    }

    // fn back_step(&mut self, co: Context, _slice: Slice<'_>, _: Option<ReturnType>) {}

    fn step(
        &mut self,
        co: Context,
        source: Weak<ParserSource>,
        parser_stepper: ParserSourceStepper,
        has_title: bool,
    ) -> ParseResult {
        let paragraph_index = self.parser_stepper.step(&mut source.);
        if let Some(paragraph) = parser_stepper.next(&mut self.source) {
            let slice = Slice::new(&*paragraph);

            if !has_title {
                let child = Title::new(paragraph_index);
                has_title = true;

                let (index, ret) = co.result_child(
                    child,
                    Title::parse,
                    move |s: &mut Self, co, _, _| s.step(co, parser_stepper, has_title),
                    slice,
                );

                self.tree.push(index);

                ret
            } else {
                todo!()
            }
        } else {
            ParseResult::Match {
                pos: paragraph_index,
                return_type: ReturnType::Null,
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

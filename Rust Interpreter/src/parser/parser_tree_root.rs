use itertools::Itertools;

use crate::parser::{parsable_vec::ParsableVec, types::ReturnType, ParserSource};

use super::{
    commands::{title::Title, Parsable, ParseTreeObj},
    context::{Context, ParseResult},
    slice::Slice,
    source::ParserSourceStepper,
    tree_writer::{indent::Indent, lint_writer::LintWriter, TreeWriter},
};
use std::any::Any;

#[derive(Debug)]
pub struct ParserTreeRoot {
    pub tree: Vec<usize>,
}

impl ParseTreeObj for ParserTreeRoot {
    fn name(&self) -> &'static str {
        "Parser Tree Root"
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Parsable for ParserTreeRoot {
    fn get_children(&self) -> Vec<usize> {
        self.tree.clone()
    }
}

impl ParserTreeRoot {
    pub fn new() -> ParserTreeRoot {
        Self { tree: Vec::new() }
    }

    pub fn parse(&mut self, co: Context, source: &mut ParserSource) -> ParseResult {
        self.step(co, source, ParserSourceStepper::new(), false)
    }

    fn step(
        &mut self,
        co: Context,
        source: &mut ParserSource,
        mut parser_stepper: ParserSourceStepper,
        mut has_title: bool,
    ) -> ParseResult {
        let paragraph_index = parser_stepper.step(source);
        if let Some(paragraph) = parser_stepper.next(source) {
            let slice = Slice::new(&*paragraph);

            if !has_title {
                let child = Title::new(paragraph_index);
                has_title = true;

                let (index, ret) = co.result_root_child(
                    child,
                    Title::parse,
                    move |s: &mut Self, cot,  source| s.step(cot, source, parser_stepper, has_title),
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
    fn write_lisp(&self, vec: &ParsableVec) -> String {
        self.tree
            .iter()
            .map(|&index| vec.get(index).write_lisp(vec))
            .join("\n\n")
    }

    fn write_lint(&self, vec: &ParsableVec, writer: &mut LintWriter, indent: u8) {
        for &index in &self.tree {
            vec.get(index).write_lint(vec, writer, indent);
        }
    }

    fn write_javascript(&self, vec: &ParsableVec, indent: Indent) -> String {
        self.tree
            .iter()
            .map(|&index| vec.get(index).write_javascript(vec, indent))
            .join("\n\n")
    }
}

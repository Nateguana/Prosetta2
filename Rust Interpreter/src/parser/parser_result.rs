use super::{commands::Paragraph, fail_reason::FailReason, types::ReturnType, ParserSource};

#[derive(Debug)]
pub struct ParserData {
    pub source: ParserSource,
    pub tree: Vec<Box<dyn Paragraph>>,
}
#[derive(Debug)]
pub struct ParserStep {
    pub pos: usize,
    pub action: ParserAction,
}

impl ParserStep {
    pub fn new(action: ParserAction, pos: usize) -> Self {
        Self { pos, action }
    }
}

#[derive(Debug)]
pub enum ParserAction {
    Move {
        child: &'static str,
    },
    StartParagraph {
        index: usize,
        child: &'static str,
    },
    Child {
        child: &'static str,
        parent: &'static str,
    },
    Matched {
        child: &'static str,
        parent: &'static str,
        return_type: ReturnType,
    },
    Failed {
        child: &'static str,
        parent: &'static str,
        reason: FailReason,
    },
    Finished,
}

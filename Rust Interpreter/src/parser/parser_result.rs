use super::{
    commands::Paragraph, fail_reason::FailReason, parsable_vec::ParsableVec, types::ReturnType,
    ParserSource,
};

#[derive(Debug)]
pub struct ParserData {
    pub source: ParserSource,
    pub tree: ParsableVec,
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
    Continue {
        child: String,
        description: String,
    },
    StartParagraph {
        index: usize,
        child: String,
    },
    Child {
        child: String,
        parent: String,
    },
    Matched {
        child: String,
        parent: String,
        return_type: ReturnType,
    },
    Failed {
        child: String,
        parent: String,
        reason: FailReason,
    },
    Finished,
}

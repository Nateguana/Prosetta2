#[derive(Debug)]
pub struct ParserData {
    // pub source: ParserSource,
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
    },
    Failed {
        child: &'static str,
        parent: &'static str,
    },
    Finished {
        data: Box<ParserData>,
    },
}

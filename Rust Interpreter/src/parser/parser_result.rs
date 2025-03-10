#[derive(PartialEq, Debug, Clone)]

pub struct ParserStep {
    pub pos: usize,
    pub parent: &'static str,
    pub action: ParserAction,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ParserAction {
    Move,
    Child(&'static str),
    Matched(&'static str),
    Failed(&'static str),
    Finished,
}

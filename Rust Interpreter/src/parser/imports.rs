use super::alias_finder::ImportParseData;

#[allow(dead_code)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Import {
    List,
    Func,
    Graph,
    Frame,
    Trig,
    Rand,
    Stamp,
    Not,
}

const IMPORTS: [ImportParseData; 8] = [
    ImportParseData::new(Import::List, *b"lis"),
    ImportParseData::new(Import::Func, *b"fun"),
    ImportParseData::new(Import::Graph, *b"gra"),
    ImportParseData::new(Import::Frame, *b"ram"),
    ImportParseData::new(Import::Trig, *b"tri"),
    ImportParseData::new(Import::Rand, *b"ran"),
    ImportParseData::new(Import::Stamp, *b"tam"),
    ImportParseData::new(Import::Not, *b"not"),
];

impl Import {
    pub fn name(&self) -> &'static str {
        match self {
            Import::List => "List",
            Import::Func => "Func",
            Import::Graph => "Graph",
            Import::Frame => "Frame",
            Import::Trig => "Trig",
            Import::Rand => "Rand",
            Import::Stamp => "Stamp",
            Import::Not => "Not",
        }
    }
    pub const fn get_all() -> &'static [ImportParseData] {
        &IMPORTS
    }
}

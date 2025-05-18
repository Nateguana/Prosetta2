use super::commands::AliasName;

#[derive(Clone)]
pub struct ImportParseData {
    pub alias: AliasName,
    pub import: Import,
    pub index: u8,
}

impl ImportParseData {
    pub const fn new(import: Import, alias: AliasName) -> Self {
        Self {
            alias,
            import,
            index: 0,
        }
    }
}

#[derive(Debug)]
pub struct ImportData {
    pub pos: usize,
    pub alias: AliasName,
    pub import: Import,
    pub length: u8,
}

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
    // Not,
}

const IMPORTS: &[ImportParseData] = &[
    ImportParseData::new(Import::List, *b"lis"),
    ImportParseData::new(Import::Func, *b"fun"),
    ImportParseData::new(Import::Graph, *b"gra"),
    ImportParseData::new(Import::Frame, *b"ram"),
    ImportParseData::new(Import::Trig, *b"tri"),
    ImportParseData::new(Import::Rand, *b"ran"),
    ImportParseData::new(Import::Stamp, *b"tam"),
    // ImportParseData::new(Import::Not, *b"not"),
];

impl Import {
    pub fn name(&self) -> &'static str {
        match self {
            Import::List => "Lists",
            Import::Func => "Functions",
            Import::Graph => "Graphics",
            Import::Frame => "Animation Frame",
            Import::Trig => "Trigonometry",
            Import::Rand => "Random",
            Import::Stamp => "Stamps",
            // Import::Not => "Not",
        }
    }
    pub const fn get_all() -> &'static [ImportParseData] {
        IMPORTS
    }
}

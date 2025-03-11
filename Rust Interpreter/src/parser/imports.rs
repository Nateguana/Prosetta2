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
    // pub fn get_all() -> &'static [(Import, &'static [u8])] {
    //     &[
    //         (Import::List, b"lis"),
    //         (Import::Func, b"fun"),
    //         (Import::Graph, b"gra"),
    //         (Import::Frame, b"fram"),
    //         (Import::Trig, b"tri"),
    //         (Import::Rand, b"ran"),
    //         (Import::Stamp, b"tam"),
    //         (Import::Not, b"not"),
    //     ]
    // }
}

use super::parser::{tree_writer::TreeAllWriter, Parser, ParserData, ParserSource};

use cap::Cap;
use std::alloc;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

#[wasm_bindgen]
pub fn get_heap_size() -> usize {
    ALLOCATOR.allocated()
}

#[wasm_bindgen]
pub struct ParserAPI {
    parser: Parser,
}

#[wasm_bindgen]
pub struct ParserRunData {
    data: ParserData,
}

// #[cfg_attr(feature = "wasm", wasm_bindgen)]
// pub struct ParserDebugAPI {
//     parser: Parser,
// }

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl ParserAPI {
    #[wasm_bindgen(constructor)]
    pub fn new(source: String) -> Self {
        let source = ParserSource::from_string(source.into());
        let parser = Parser::new(source);
        Self { parser }
    }

    // pub fn debug(&mut self, source: &str) -> ParserDebugAPI {

    // }

    pub fn run(self) -> ParserRunData {
        ParserRunData {
            data: self.parser.run(),
        }
    }
}

#[wasm_bindgen]
impl ParserRunData {
    pub fn get_javascript(&self) -> String {
        TreeAllWriter::write_all_javascript(&self.data.tree)
    }

    pub fn get_lisp_like(&self) -> String {
        TreeAllWriter::write_all_lisp(&self.data.tree)
    }
    // pub fn get_html(&self) -> String {
    //     let iter = self.data.source.get_iter();
    //     let mut lint = SyntaxLinter::<HTMLRenderer>::new();
    //     lint.write(&self.data.exprs, &self.data.stat_starts, iter);
    //     String::from_utf8_lossy(&lint.into_data()).to_string()
    // }
    // pub fn get_highlights(&self) -> Vec<Highlight> {
    //     let iter = self.data.source.get_iter();
    //     let mut lint = SyntaxLinter::<LineRenderer>::new();
    //     lint.write(&self.data.exprs, &self.data.stat_starts, iter);
    //     lint.into_data()
    // }
    // pub fn get_imports(&self) -> Vec<Import> {
    //     self.data.imports.clone()
    // }
    // pub fn get_triggers(&self) -> String {
    //     word_trigger_writer::write(&self.data.trigger_word_data.word_triggers)
    // }
}
//wasm-pack build . -F wasm

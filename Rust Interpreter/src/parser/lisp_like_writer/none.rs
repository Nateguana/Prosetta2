use crate::parser::commands::none::NoneStart;

use super::{commands::none::None, LispWriter};

impl LispWriter for None {
    fn write_lisp(&self) -> String {
        format!("(TODO)")
    }
}

impl LispWriter for NoneStart {
    fn write_lisp(&self) -> String {
        unreachable!()
    }
}

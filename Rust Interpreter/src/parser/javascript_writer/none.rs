use crate::parser::commands::none::NoneStart;

use super::{commands::none::None, JavascriptWriter};

impl JavascriptWriter for None {
    fn write_javascript(&self) -> String {
        format!("TODO()")
    }
}


impl JavascriptWriter for NoneStart {
    fn write_javascript(&self) -> String {
        unreachable!()
    }
}
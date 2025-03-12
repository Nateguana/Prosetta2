use super::{commands::none::None, LispWriter};

impl LispWriter for None {
    fn write_lisp(&self) -> String {
        format!("(TODO)")
    }
}

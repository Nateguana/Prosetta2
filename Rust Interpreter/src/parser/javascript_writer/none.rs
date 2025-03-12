use super::{commands::none::None, JavascriptWriter};

impl JavascriptWriter for None {
    fn write_javascript(&self) -> String {
        format!("TODO()")
    }
}

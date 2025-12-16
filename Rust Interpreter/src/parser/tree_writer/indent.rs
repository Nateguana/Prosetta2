#[derive(Clone,Copy)]
pub struct Indent {
    indent: u8,
}


impl Indent {
    pub fn new() -> Self {
        Self { indent: 0 }
    }
    pub fn str(&self) -> String {
        " ".repeat((self.indent as usize) * 4)
    }
    pub fn add(&self) -> Self {
        Self {
            indent: self.indent + 1,
        }
    }
}

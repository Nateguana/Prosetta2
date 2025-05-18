use super::{commands::paragraph_start::ParagraphStart, JavascriptWriter};


impl JavascriptWriter for ParagraphStart {
    fn write_javascript(&self, _indent: u8) -> String {
        let children = self.children.read();
        let mut ret = format!("// Paragraph {}", self.index);

        for child in children.iter() {
            ret += &format!("\n{}", child.write_javascript(0));
        }

        ret
    }
}

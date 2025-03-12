use super::{LispWriter, ParagraphType};
use itertools::Itertools;

impl LispWriter for ParagraphType {
    fn write_lisp(&self) -> String {
        match self {
            ParagraphType::Title(title) => title.write_lisp(),
            ParagraphType::Regular(commands) => commands
                .into_iter()
                .map(|comm| comm.write_lisp())
                .join("\n"),
        }
    }
}

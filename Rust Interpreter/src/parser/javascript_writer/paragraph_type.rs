use super::{JavascriptWriter, ParagraphType};
use itertools::Itertools;

impl JavascriptWriter for ParagraphType {
    fn write_javascript(&self) -> String {
        match self {
            ParagraphType::Title(title) => title.write_javascript(),
            ParagraphType::Regular(commands) => commands
                .into_iter()
                .map(|comm| comm.write_javascript())
                .join("\n"),
        }
    }
}

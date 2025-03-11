mod title;
use super::commands;

pub trait JavascriptWriter {
    fn write(&self) -> String;
}

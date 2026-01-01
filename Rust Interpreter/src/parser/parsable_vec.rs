use crate::parser::commands::{none::NoneCommand, Parsable};

pub struct ParsableVec {
    inner: Vec<Box<dyn Parsable>>,
    empty: NoneCommand,
}

impl ParsableVec {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            empty: NoneCommand,
        }
    }

    pub fn push(&mut self, element: Box<dyn Parsable>) -> usize {
        self.inner.push(element);
        self.inner.len()
    }

    pub fn drain(&mut self, to: usize) {
        self.inner.drain(to - 1..);
    }

    pub fn get_mut(&mut self, index: usize) -> &mut dyn Parsable {
        self.inner
            .get_mut(index - 1)
            .map_or(&mut self.empty, |e| e.as_mut())
    }

    pub fn get(&self, index: usize) -> &dyn Parsable {
        self.inner
            .get(index - 1)
            .map_or(&self.empty, |e| e.as_ref())
    }

    pub fn split(
        &mut self,
        index: usize,
    ) -> (ParseableVecSplit, &mut dyn Parsable, ParseableVecSplit) {
        let (before, after) = self.inner.split_at_mut(index - 1);
        let (node, after) = after.split_first_mut().unwrap();
        (
            ParseableVecSplit {
                inner: before,
                index: 1,
                empty: NoneCommand,
            },
            node,
            ParseableVecSplit {
                inner: after,
                index: index,
                empty: NoneCommand,
            },
        )
    }

    pub fn into_root(self) -> Box<dyn Parsable> {
        self.inner[0]
    }
}

pub struct ParseableVecSplit<'a> {
    inner: &'a mut [Box<dyn Parsable>],
    index: usize,
    empty: NoneCommand,
}

impl<'a> ParseableVecSplit<'a> {
    pub fn new() {}

    pub fn get_mut(&mut self, index: usize) -> &mut dyn Parsable {
        self.inner
            .get_mut(index - self.index)
            .map_or(&mut self.empty, |e| e.as_mut())
    }

    pub fn get(&self, index: usize) -> &dyn Parsable {
        self.inner
            .get(index - self.index)
            .map_or(&self.empty, |e| e.as_ref())
    }
}

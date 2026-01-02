use crate::parser::commands::{none::NoneCommand, Parsable};

pub struct ParsableVec {
    inner: Vec<Box<dyn Parsable>>,
    dirty: bool,
    empty: NoneCommand,
}

impl ParsableVec {
    pub fn new() -> Self {
        Self {
            inner: vec![Box::new(NoneCommand)],
            empty: NoneCommand,
            dirty: false,
        }
    }

    pub fn push(&mut self, element: Box<dyn Parsable>) -> usize {
        *self.inner.last_mut().unwrap() = element;
        self.inner.push(Box::new(NoneCommand));
        self.inner.len() - 1
    }

    pub fn update(&mut self) {
        if self.dirty {
            self.dirty = false;
            self.inner.push(Box::new(NoneCommand));
        }
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

    pub fn split(&mut self, index: usize) -> (&mut dyn Parsable, ParseableVecSplit) {
        let (before, after) = self.inner.split_at_mut(index - 1);
        let (node, after) = after.split_first_mut().unwrap();
        let vec_split = ParseableVecSplit {
            before,
            after,
            index,
            empty: NoneCommand,
            dirty_flag: &mut self.dirty,
        };

        (node.as_mut(), vec_split)
    }

    pub fn into_root(self) -> Box<dyn Parsable> {
        self.inner.into_iter().next().unwrap()
    }
}

pub struct ParseableVecSplit<'a> {
    before: &'a mut [Box<dyn Parsable>],
    after: &'a mut [Box<dyn Parsable>],
    dirty_flag: &'a mut bool,
    index: usize,
    empty: NoneCommand,
}

impl<'a> ParseableVecSplit<'a> {
    pub fn get_mut(&mut self, index: usize) -> &mut dyn Parsable {
        match index.cmp(&self.index) {
            std::cmp::Ordering::Less => self.before.get_mut(index - 1),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => self.after.get_mut(index - self.index),
        }
        .map_or(&mut self.empty, |e| e.as_mut())
    }

    pub fn get(&self, index: usize) -> &dyn Parsable {
        match index.cmp(&self.index) {
            std::cmp::Ordering::Less => self.before.get(index - 1),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => self.after.get(index - self.index),
        }
        .map_or(&self.empty, |e| e.as_ref())
    }

    pub fn push(&mut self, element: Box<dyn Parsable>) -> usize {
        *self.dirty_flag = true;
        *self.after.last_mut().unwrap() = element;
        self.index + self.after.len()
    }
}

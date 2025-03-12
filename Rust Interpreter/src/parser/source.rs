use std::{
    collections::VecDeque,
    fmt::Debug,
    io::{stdin, BufRead},
    mem,
    // ops::Deref,
};

use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
// use streaming_iterator::StreamingIterator;

// pub type ParserSourceIter<'a> = Flatten<std::vec::IntoIter<Box<dyn Iterator<Item = &'a u8> + 'a>>>;

// macro_rules! make_iter {
//     ($expr:expr) => {
//         Box::new($expr) as Box<dyn iter::Iterator<Item = &u8>>
//     };
// }

#[derive(Debug)]
enum Source {
    Stdin,
    File,
    String(Vec<u8>),
}

#[derive(Debug)]
pub struct ParserSource {
    sources: RwLock<VecDeque<Source>>,
    paragraphs: RwLock<Vec<Vec<u8>>>,
}

impl ParserSource {
    pub fn new() -> Self {
        Self {
            sources: RwLock::new(VecDeque::new()),
            paragraphs: RwLock::new(Vec::new()),
        }
    }
    pub fn from_stdin() -> Self {
        Self::new().add_stdin()
    }
    pub fn from_string(str: Vec<u8>) -> Self {
        Self::new().add_string(str)
    }
}

impl ParserSource {
    pub fn add_stdin(self) -> Self {
        self.sources.write().push_back(Source::Stdin);
        self
    }

    pub fn add_string(self, str: Vec<u8>) -> Self {
        self.sources.write().push_back(Source::String(str));
        self
    }

    pub fn get_mut_iter<'a>(&'a self) -> MutParserSourceIter<'a> {
        MutParserSourceIter::new(self)
    }

    pub fn get_iter<'a>(&'a self) -> ParserSourceIter<'a> {
        ParserSourceIter::new(self)
    }
}

pub struct ParserSourceIter<'a> {
    source: &'a ParserSource,
    paragraph_index: usize,
}

pub struct MutParserSourceIter<'a> {
    inner: ParserSourceIter<'a>,
    source_index: usize,
}

impl<'a> Iterator for MutParserSourceIter<'a> {
    type Item = Vec<u8>;

    // fn advance(&mut self) {
    //     if self.paragraph_index >= self.source.paragraphs.borrow().len() {
    //         match self.source.sources.pop_front() {
    //             Some(source) => match source {
    //                 Source::Stdin => self.get_from_stdin(),
    //                 Source::File => todo!(),
    //                 Source::String(str) => self.get_from_string(str),
    //             },
    //             None => {}
    //         }
    //     }
    //     self.paragraph_index += 1;
    // }

    // fn get<'b>(&'b self) -> Option<impl Deref<Target = [u8]> + Send + 'b> {
    //     let par_ref = self.source.paragraphs.borrow();
    //     let par_ref2 = Ref::map(par_ref, |par| {
    //         par.get(self.paragraph_index - 1)
    //             .map_or(&[] as &[u8], |par| &**par)
    //     });
    //     par_ref2.is_empty().then(|| par_ref2)
    // }

    // fn get_paragraph<'b>(&'b mut self, par: &'b Vec<Vec<u8>>, index: usize) -> Option<&'b [u8]> {
    //     par.get(index - 1).map(|e| &**e)
    // }

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        if self.inner.paragraph_index >= self.inner.source.paragraphs.read().len() {
            let should_remove = match self.inner.source.sources.read().get(0) {
                Some(source) => match source {
                    Source::Stdin => self.get_from_stdin(),
                    Source::File => todo!(),
                    Source::String(buf) => self.get_from_string(buf.clone()),
                },
                None => false,
            };
            if should_remove {
                self.inner.source.sources.write().pop_front();
            }
        }
        let ret = self
            .inner
            .source
            .paragraphs
            .read()
            .get(self.inner.paragraph_index)
            .cloned();
        self.inner.paragraph_index += 1;
        ret
    }
}

impl<'a> MutParserSourceIter<'a> {
    fn get_from_stdin(&mut self) -> bool {
        let mut stdin = stdin().lock();
        println!("Input text to be parsed:");
        // let mut has_input = false;
        let mut paragraph = Vec::new();
        loop {
            let mut new_input = Vec::new();
            let has_failed = stdin.read_until(b'\n', &mut new_input).is_err();

            // remove newlines if it exists
            while let Some(b'\n' | b'\r') = new_input.last() {
                new_input.pop();
            }

            //if empty line or stdin closed
            if has_failed || new_input.trim().len() == 0 {
                let has_input = paragraph.len() > 0;
                self.inner
                    .source
                    .paragraphs
                    .write()
                    .push(mem::take(&mut paragraph));
                return !has_input;
            }

            // add line to paragraph
            if paragraph.len() != 0 {
                paragraph.push(b'\n');
            }
            paragraph.append(&mut new_input);
            // has_input = true;
        }
    }

    fn get_from_string(&mut self, buf: Vec<u8>) -> bool {
        let mut paragraph = Vec::new();
        let mut start = 0;
        let mut last_empty = false;

        for index in buf.iter().positions(|&e| e == b'\n' || e == b'\r') {
            let line = &buf[start..index - 1];
            let is_empty = line.trim().len() == 0 && buf[index] == b'\n';
            if is_empty && last_empty {
                paragraph.push_str(line);
            }
            last_empty = is_empty;
            start = index + 1;
        }
        true
    }
}

impl<'a> Iterator for ParserSourceIter<'a> {
    type Item = MappedRwLockReadGuard<'a, [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        let par_ref = self.source.paragraphs.read();
        let par_ref2 = RwLockReadGuard::map(par_ref, |par| {
            par.get(self.paragraph_index)
                .map_or(&[] as &[u8], |par| &**par)
        });
        self.paragraph_index += 1;
        par_ref2.is_empty().then(|| par_ref2)
    }
}

impl<'a> MutParserSourceIter<'a> {
    fn new(source: &'a ParserSource) -> Self {
        Self {
            inner: ParserSourceIter::new(source),
            source_index: 0,
        }
    }
}

impl<'a> ParserSourceIter<'a> {
    fn new(source: &'a ParserSource) -> Self {
        Self {
            source,
            paragraph_index: 0,
        }
    }
}

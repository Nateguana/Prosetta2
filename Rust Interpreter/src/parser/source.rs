use std::{
    collections::VecDeque,
    fmt::Debug,
    io::{stdin, BufRead},
    mem,
    // ops::Deref,
};

use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
// use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
// use super::rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
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
    sources: VecDeque<Source>,
    paragraphs: Vec<Vec<u8>>,
}

pub struct ParserSourceStepper {
    paragraph_index: usize,
    source_index: usize,
}

impl ParserSource {
    pub fn new() -> Self {
        Self {
            sources: VecDeque::new(),
            paragraphs: Vec::new(),
        }
    }
    pub fn from_stdin() -> Self {
        Self::new().add_stdin()
    }

    pub fn from_string(str: Vec<u8>) -> Self {
        Self::new().add_string(str)
    }

    pub fn get_iter(&self) -> impl Iterator<Item = &[u8]> {
        self.paragraphs.iter().map(|e| e.as_slice())
    }
}

impl ParserSource {
    pub fn add_stdin(mut self) -> Self {
        self.sources.push_back(Source::Stdin);
        self
    }

    pub fn add_string(mut self, str: Vec<u8>) -> Self {
        self.sources.push_back(Source::String(str));
        self
    }
}

impl ParserSourceStepper {
    pub fn new() -> Self {
        Self {
            paragraph_index: 0,
            source_index: 0,
        }
    }

    pub fn step(&mut self, parser_source: &mut ParserSource) {
        if self.paragraph_index >= parser_source.paragraphs.len() {
            let should_remove = match parser_source.sources.get_mut(0) {
                Some(source) => match source {
                    Source::Stdin => self.get_from_stdin(parser_source),
                    Source::File => todo!(),
                    Source::String(buf) => self.get_from_string(buf),
                },
                None => false,
            };
            if should_remove {
                parser_source.sources.pop_front();
            }
        }
    }

    pub fn next<'a>(&mut self, parser_source: &'a ParserSource) -> Option<&'a [u8]> {
        let ret = parser_source
            .paragraphs
            .get(self.paragraph_index)
            .map(|e| e.as_slice());
        self.paragraph_index += 1;
        ret
    }
}

impl ParserSourceStepper {
    fn get_from_stdin(&mut self, source: &mut ParserSource) -> bool {
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
                source.paragraphs.push(mem::take(&mut paragraph));
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

    fn get_from_string(&mut self, buf: &mut Vec<u8>) -> bool {
        let mut paragraph = Vec::new();
        let mut start = 0;
        let mut last_empty = false;

        // while let Some() = buf.iter().rposition(|n| e == b'\n' || e == b'\r'){

        // }
        for index in buf.iter().positions(|&e| e == b'\n' || e == b'\r') {
            let line = &buf[start..index - 1];
            let is_empty = line.trim().len() == 0 && buf[index] == b'\n';
            if is_empty && last_empty {
                paragraph.push_str(line);
            }
            last_empty = is_empty;
            start = index + 1;
        }
        buf.drain(..);
        true
    }
}

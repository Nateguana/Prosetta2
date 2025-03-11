use std::{
    collections::VecDeque,
    fmt::Debug,
    io::{stdin, BufRead},
    mem,
};

use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use streaming_iterator::StreamingIterator;

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

    pub fn get_mut_iter<'a>(&'a mut self) -> MutParserSourceIter<'a> {
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
    source: &'a mut ParserSource,
    source_index: usize,
    paragraph_index: usize,
}

impl<'a> StreamingIterator for MutParserSourceIter<'a> {
    type Item = Vec<u8>;

    fn advance(&mut self) {
        if self.paragraph_index >= self.source.paragraphs.len() {
            match self.source.sources.pop_front() {
                Some(source) => match source {
                    Source::Stdin => self.get_from_stdin(),
                    Source::File => todo!(),
                    Source::String(str) => self.get_from_string(str),
                },
                None => {}
            }
        }
        self.paragraph_index += 1;
    }

    fn get(&self) -> Option<&Self::Item> {
        self.source.paragraphs.get(self.paragraph_index - 1)

        // let len = self.source.sources.len();

        // while self.paragraph_index < self.source.paragraphs.len(){

        // }
        //     || self.source_index < self.source.sources.len()
        // {
        //     if let Some(buf) = self.source.paragraphs.get(self.paragraph_index) {
        //         self.paragraph_index += 1;
        //         return Some(buf);
        //     }
        //     if let Some() = self.source.sources.get(self.source_index) {
        //     } else {
        //         self.source_index += 1;
        //     }
        // }
        // while self.index < len {
        //     let result = match &mut self.source.sources[self.index] {
        //         Source::Stdin {
        //             buf,
        //             paragraphs,
        //             first,
        //         } => self
        //             .source
        //             .get_from_stdin(buf, paragraphs, *first, self.inner_index),
        //         Source::File => todo!(),
        //         Source::String { str, paragraphs } => {
        //             // // if getting before new line is set - return nothing
        //             // if *first {
        //             //     &[]
        //             // } else {
        //             //     &str
        //             // }
        //             todo!()
        //         }
        //     };
        // self.inner_index += 1;
        // if result.is_some() {
        //     return result;
        // } else {
        //     self.inner_index = 0;
        //     self.index += 1;
        // }
        // None
    }
}

impl<'a> MutParserSourceIter<'a> {
    fn get_from_stdin(&mut self) {
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
                self.source.paragraphs.push(mem::take(&mut paragraph));
                return;
            }

            // add line to paragraph
            if paragraph.len() != 0 {
                paragraph.push(b'\n');
            }
            paragraph.append(&mut new_input);
            // has_input = true;
        }
    }

    fn get_from_string(&mut self, buf: Vec<u8>) {
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
            // if buf[slice] == b'\n' {
            //     paragraph
            // }
            // if is empty line add to paragraph
            // if index.trim().len() == 0 {
            //     paragraphs.push((last_start, last_line - 1));
            //     last_start = last_line + 1;
            // }
            // last_line += index.len();
        }
        // self.sources.push(Source::String { str, paragraphs });

        // self
    }
}

impl<'a> Iterator for ParserSourceIter<'a> {
    type Item = &'a Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.source.paragraphs.get(self.paragraph_index);
        self.paragraph_index += 1;
        ret
    }
}

impl<'a> MutParserSourceIter<'a> {
    fn new(source: &'a mut ParserSource) -> Self {
        Self {
            source,
            source_index: 0,
            paragraph_index: 0,
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

//     pub fn add_string(mut self, str: Vec<u8>) -> Self {
//         // if last is not newline - add it
//         // if does_str_need_newline(&str) {
//         //     str.push(b'\n');
//         // }
//         let mut paragraph = Vec::new();
//         let mut empty_line = false;
//         let mut first_text = false;
//         for slice in str.split_str("\n") {
//             // if is empty line -- change empty line
//             if slice.trim().len() == 0 {
//                 empty_line = true;
//             } else {
//                 // if empty line found -- make new buffer
//                 if empty_line && first_text {
//                     // println!("{:?}", paragraph);
//                     self.sources.push(Source::String {
//                         str: mem::take(&mut paragraph),
//                         first: true,
//                     });
//                 }
//                 empty_line = false;
//                 first_text = true;
//             }
//             paragraph.push_str(slice);
//             paragraph.push(b'\n');
//         }
//         // println!("{:?}", paragraph);
//         self.sources.push(Source::String {
//             str: paragraph,
//             first: true,
//         });

//         self
//     }
// }

// impl<'a> ParserSource<'a> {
//     pub fn get_line<'b>(&'b self) -> &'b [u8] {
//         match &self.sources[self.index] {
//             Source::Stdin { start, buf, .. } => &buf[*start..],
//             Source::File => todo!(),
//             Source::String { str, first } => {
//                 // if getting before new line is set - return nothing
//                 if *first {
//                     &[]
//                 } else {
//                     &str
//                 }
//             }
//         }
//     }
//     pub fn new_line<'b>(&'b mut self) -> Option<&'b [u8]> {
//         loop {
//             if self.index >= self.sources.len() {
//                 return None;
//             }
//             let has_failed = match &mut self.sources[self.index] {
//                 Source::Stdin { source, start, buf } => {
//                     if let Some(stdin) = source {
//                         Self::get_from_stdin(stdin, start, buf)
//                     } else {
//                         true
//                     }
//                 }
//                 Source::File => todo!(),
//                 Source::String { first, .. } => {
//                     let ret = *first;
//                     *first = false;
//                     !ret
//                 }
//             };
//             if has_failed {
//                 self.index += 1;
//             } else {
//                 return Some(self.get_line());
//             }
//         }
//     }

//     pub fn drop_input(&mut self) {
//         for s in &mut self.sources {
//             if let Source::Stdin { source, .. } = s {
//                 *source = None;
//             }
//         }
//     }
//     pub fn get_iter(&self) -> ParserSourceIter {
//         let mut ret = Vec::new();
//         let mut add_newline = false;
//         for s in &self.sources {
//             if add_newline {
//                 ret.push(make_iter!(iter::once(&b'\n')));
//             }
//             let iter;
//             (iter, add_newline) = match s {
//                 Source::Stdin { buf, .. } => (make_iter!(buf.iter()), false),
//                 Source::File => todo!(),
//                 Source::String { str, .. } => (make_iter!(str.iter()), does_str_need_newline(str)),
//             };
//             ret.push(iter);
//         }
//         ret.into_iter().flatten()
//     }
// }

// impl<'a> ParserSource<'a> {
//     /// get input from stdin stoping on 0 len input
//     /// returns has_failed
//     fn get_from_stdin(stdin: &mut StdinLock<'a>, start: &mut usize, buf: &mut Vec<u8>) -> bool {
//         println!("Input text to be parsed:");
//         let mut has_input = false;
//         // let mut has_first_empty = false;
//         *start = buf.len();
//         loop {
//             let mut new_input = Vec::new();
//             let has_failed = stdin.read_until(b'\n', &mut new_input).is_err();

//             // remove newlines if it exists
//             while !new_input.is_empty() {
//                 let last = *new_input.last().unwrap();
//                 if last == b'\n' || last == b'\r' {
//                     new_input.pop();
//                 } else {
//                     break;
//                 }
//             }

//             if has_failed || new_input.len() == 0 {
//                 // if has_first_empty {
//                 buf.push(b'\n');
//                 return !has_input;
//             }

//             has_input = true;
//             if buf.len() == 0 {
//                 *buf = new_input;
//             } else {
//                 buf.append(&mut new_input);
//             }
//             buf.push(b'\n');
//         }
//     }
// }

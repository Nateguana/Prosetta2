use core::str;
use std::fmt::Debug;

use super::close_data;

fn is_word_arg_character(char: u8) -> bool {
    char.is_ascii_alphabetic() || char == b'\'' || char == b'-'
}

#[derive(PartialEq, Clone, Copy)]
pub struct Slice<'a> {
    ///the string itself
    pub str: &'a [u8],
    ///the position relative to the buffer
    pub pos: usize,
}

impl<'a> Debug for Slice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slice")
            .field("str", &str::from_utf8(&self.str).unwrap())
            .field("pos", &self.pos)
            .finish()
    }
}

impl<'a> Slice<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Slice { str: buf, pos: 0 }
    }

    pub fn empty() -> Self {
        Slice { str: &[], pos: 0 }
    }
    ///the length of the slice
    ///the same as .str.len()
    pub fn len(self) -> usize {
        self.str.len()
    }
    ///the end of the slice relative to the buffer
    pub fn end(self) -> usize {
        self.pos + self.str.len()
    }

    ///returns a new slice that is shortened by offset
    pub fn offset(self, offset: usize) -> Slice<'a> {
        Slice {
            str: &self.str[offset..],
            pos: self.pos + offset,
        }
    }
    pub fn slice(self, start: usize, end: usize) -> Slice<'a> {
        Slice {
            str: &self.str[start..end],
            pos: self.pos + start,
        }
    }
    pub fn split(self, start: usize, end: usize) -> (Slice<'a>, Slice<'a>) {
        (self.slice(start, end), self.offset(end))
    }
}

impl<'a> Slice<'a> {
    pub fn find_slice(
        self,
        find_pred: impl Fn(&[u8]) -> bool,
        end_pred: Option<impl Fn(&[u8]) -> bool>,
    ) -> (Slice<'a>, Slice<'a>) {
        let mut start = 0;
        let mut end = self.str.len();
        while start < self.len() && !(find_pred)(&self.str[start..]) {
            start += 1;
        }
        if let Some(pred) = end_pred {
            end = start + 1;
            while end < self.len() && (pred)(&self.str[end..]) {
                end += 1;
            }
        }
        self.split(start, end)
    }

    pub fn get_next_word_arg(mut self) -> (Slice<'a>, Slice<'a>) {
        let func = |str: &[u8]| is_word_arg_character(str[0]);

        let mut slice = Slice::empty();
        while self.len() > 0 {
            (slice, self) = self.find_slice(func, Some(func));

            if slice.str.len() <= 255 {
                break;
            }
        }
        (slice, self)
    }

    pub fn get_next_word_arg_3(mut self) -> (Slice<'a>, Slice<'a>) {
        let mut slice = Slice::empty();
        while self.len() > 0 {
            (slice, self) = self.get_next_word_arg();

            if slice.str.len() >= 3 {
                break;
            }
        }
        (slice, self)
    }

    pub fn get_next_line(self) -> (Slice<'a>, Slice<'a>) {
        let mut start = 1;
        while start < self.len() {
            while start < self.len() && self.str[start - 1] != b'\n' {
                start += 1;
            }
            return self.split(0, start);
        }
        self.split(0, self.len())
    }
    pub fn get_next_slice(self) -> (Slice<'a>, Slice<'a>) {
        let mut start = 0;
        while start < self.len()
            && !close_data::is_valid_word_char(self.str[start])
            && !close_data::is_valid_close_char(self.str[start])
            && !close_data::is_non_close_but_still_single(self.str[start])
        {
            start += 1;
        }

        // find end of word
        let mut end = start;

        let close_data = close_data::get_close_data(&self.str[start..]);
        if close_data.close_length != 0 {
            end += close_data.close_length as usize;
        } else if end < self.len() && close_data::is_non_close_but_still_single(self.str[start]) {
            end += 1;
        } else {
            while end < self.len()
                && close_data::is_valid_word_char(self.str[end])
                && !close_data::is_word_stopper(&self.str[end..])
            {
                end += 1;
            }
        }
        self.split(start, end)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Slice<'a> {
    ///the string itself
    pub str: &'a [u8],
    ///the position relative to the buffer
    pub pos: usize,
}

impl<'a> Slice<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Slice { str: buf, pos: 0 }
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
            while end <= self.len() && (pred)(&self.str[start..]) {
                end += 1;
            }
        }
        self.split(start, end)
    }

    pub fn get_next_word_arg(mut self) -> Option<(Slice<'a>, Slice<'a>)> {
        let func = |str: &[u8]| is_word_arg_character(str[0]);

        let mut slice;
        while self.len() > 0 {
            (slice, self) = self.find_slice(func, Some(func));

            if slice.str.len() > 3 && slice.str.len() <= 255 {
                return Some((slice, self));
            }
        }
        None
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
}

fn is_word_arg_character(char: u8) -> bool {
    char.is_ascii_alphabetic() || char == b'\'' || char == b'-'
}

use std::{
    any::Any,
    mem::{self},
};

use bstr::{ByteSlice, ByteVec};
// use parking_lot::{Mutex, MutexGuard};

use super::{
    close_data, CloseData, Command, Context, FailReason, Import, Paragraph, Parseable, ReturnType,
    RwLock, Slice,
};

#[derive(Debug)]
pub struct AuthorData {
    pub name: Vec<u8>,
    pub pos: usize,
    pub length: usize,
}
#[derive(Debug)]
pub struct ImportData {
    pub name: Import,
    pub pos: usize,
    pub length: u8,
}

#[derive(Default, Debug)]
pub struct TitleData {
    /// the poem title
    pub title: Vec<u8>,
    /// the length of the poem title in poem
    /// (self.title is trimmed)
    pub title_length: usize,
    /// the author names
    pub authors: Vec<AuthorData>,
    // the imports: (type, position, length)
    pub imports: Vec<ImportData>,
    // the start of "by"
    pub by_start: usize,
}

#[derive(Default, Debug)]
pub struct Title {
    pub inner: RwLock<TitleData>,
}

impl Title {
    ///add title data and returns slice after by
    async fn find_title<'a>(&self, co: &impl Context, slice: Slice<'a>) -> Slice<'a> {
        let mut curr_slice = slice;
        let mut space: &[u8] = b"";
        loop {
            let (title, rest) = curr_slice.get_next_line();

            // add title
            {
                let mut this = self.inner.write();
                this.title.push_str(title.str.trim());
                this.title.push_str(space);
                this.title_length = rest.pos;
            }

            co.step_continue(self, rest.pos).await;

            space = b"\n";
            // no more text
            if rest.len() == 0 {
                return rest;
            }

            // find "by"
            let (word, rest2) = rest.get_next_word_arg();

            if word.str == b"by" {
                self.inner.write().by_start = word.pos;
                return rest2;
            }

            curr_slice = rest;
        }
    }

    async fn parse_authors(&self, co: &impl Context, slice: Slice<'_>) {
        let mut parsed_first = false;
        let mut curr_slice = slice;
        let mut sep: &[u8] = b"";
        let mut author_data = AuthorData {
            name: Vec::new(),
            pos: slice.pos,
            length: 0,
        };
        while curr_slice.len() > 0 {
            let slice;
            (slice, curr_slice) = curr_slice.get_next_slice();
            co.step_continue(self, slice.pos).await;
            // if is separator
            if Self::is_separator(slice.str).close_count > 0 {
                if author_data.name.len() > 0 {
                    sep = b"";
                    parsed_first = true;
                    self.inner.write().authors.push(mem::replace(
                        &mut author_data,
                        AuthorData {
                            name: Vec::new(),
                            pos: slice.pos,
                            length: 0,
                        },
                    ));
                }
            //author name
            } else {
                author_data.name.push_str(sep);
                author_data.name.push_str(slice.str);
                author_data.length = slice.end() - author_data.pos;
                sep = b" ";
                if parsed_first {
                    //find_imports()
                }
            }
        }
        // add last author
        if author_data.name.len() > 0 {
            self.inner.write().authors.push(author_data);
        }
    }

    fn is_separator(str: &[u8]) -> CloseData {
        if str.len() >= 3 && str == b"and" {
            CloseData {
                close_count: 1,
                close_length: 3,
                only_forced: true,
            }
        } else if str.len() >= 1 && str == b"&" {
            CloseData {
                close_count: 1,
                close_length: 1,
                only_forced: true,
            }
        } else {
            close_data::get_close_data(str)
        }
    }
}

impl Parseable for Title {
    fn new() -> Self {
        Default::default()
    }

    fn name(&self) -> &'static str {
        "Title"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait::async_trait]
impl Command for Title {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        let curr_slice = self.find_title(&co, slice).await;
        self.parse_authors(&co, curr_slice).await;
        Ok((slice.end(), ReturnType::Null))
    }
}

impl Paragraph for Title {}

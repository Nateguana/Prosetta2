use std::{any::Any, mem};

use bstr::{ByteSlice, ByteVec};
// use parking_lot::{Mutex, MutexGuard};

use super::{
    close_data, CloseData, Context, FailReason, Import, Paragraph, Parsable, ParseTreeObj,
    ReturnType, RwLock, Slice, Step_Continue,
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
    /// title is trimmed
    pub title_length: usize,
    /// the author names
    pub authors: Vec<AuthorData>,
    // the imports: (type, position, length)
    pub imports: Vec<ImportData>,
    //
    pub by_section_length: usize,
}

#[derive(Default, Debug)]
pub struct Title {
    pub inner: RwLock<TitleData>,
}

impl Title {
    pub fn new() -> Self {
        Default::default()
    }
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
                this.title_length += title.end();
            }

            if space.len() > 1 {
                Step_Continue!(
                    co,
                    self,
                    rest.pos,
                    "{} did not find keyword by so added line to title"
                );
            } else {
                Step_Continue!(co, self, rest.pos, "{} added first line to title");
            }

            space = b"\n";
            // no more text
            if rest.len() == 0 {
                return rest;
            }

            // find "by"
            let (word, rest2) = rest.get_next_word_arg();

            if word.str == b"by" {
                self.inner.write().by_section_length = 2;
                // self.inner.write().by_start = word.pos;
                return rest2;
            }

            // let mut test = Vec::new();

            // std::io::Write::write(&mut test, b"this should show up").unwrap();

            // black_box(test);

            // black_box(unsafe {
            //     black_box(std::arch::asm!("mov esi, esi"));
            // });

            curr_slice = rest;
        }
    }

    async fn parse_authors(&self, co: &impl Context, mut curr_slice: Slice<'_>) {
        let mut parsed_first = false;
        let mut sep: &[u8] = b"";
        let mut author_data = AuthorData {
            name: Vec::new(),
            pos: curr_slice.pos,
            length: 0,
        };
        while curr_slice.len() > 0 {
            let slice;
            (slice, curr_slice) = curr_slice.get_next_slice();
            // if is separator

            // println!("{}", str::from_utf8(slice.str).unwrap());
            if Self::is_separator(slice.str).close_count > 0 {
                if author_data.name.len() > 0 {
                    sep = b"";
                    let old_author = mem::replace(
                        &mut author_data,
                        AuthorData {
                            name: Vec::new(),
                            pos: 0,
                            length: 0,
                        },
                    );
                    {
                        let mut this = self.inner.write();
                        this.authors.push(old_author);
                        this.by_section_length = slice.pos - this.title_length;
                    }
                    Step_Continue!(co, self, slice.pos, "{} parsed an author");
                    if parsed_first {
                        let inner_lock = self.inner.read();
                        let name = inner_lock.authors.last().unwrap().name.as_slice();
                        Self::find_imports(name);
                        Step_Continue!(co, self, slice.pos, "{} parsed imports for author");
                    }
                    parsed_first = true;
                }
            //author name
            } else {
                if author_data.pos == 0 {
                    author_data.pos = slice.pos;
                }
                author_data.name.push_str(sep);
                author_data.name.push_str(slice.str);
                author_data.length = slice.end() - author_data.pos;
                sep = b" ";
            }
        }
        // add last author
        {
            let mut this = self.inner.write();
            if author_data.name.len() > 0 {
                this.authors.push(author_data);
            }
            this.by_section_length = curr_slice.end() - this.title_length;
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

    fn find_imports(str: &[u8]) {}
}

impl ParseTreeObj for Title {
    fn name(&self) -> &'static str {
        "Title"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait::async_trait]
impl Parsable for Title {
    async fn try_parse(
        &self,
        co: impl Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        let curr_slice = self.find_title(&co, slice).await;
        if curr_slice.len() > 0 {
            Step_Continue!(
                co,
                self,
                curr_slice.pos,
                "{} found author section with the keyword by"
            );
            self.parse_authors(&co, curr_slice).await;
        } else {
            Step_Continue!(co, self, curr_slice.pos, "{} never found keyword by");
        }
        Ok((slice.end(), ReturnType::Null))
    }
}

impl Paragraph for Title {}

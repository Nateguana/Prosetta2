use std::{any::Any, mem};

use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use std::str;

use super::{
    close_data, CloseData, Context, FailReason, Import, ImportData, ImportFinder, Indent,
    LintColor, LintWriter, Paragraph, Parsable, ParseTreeObj, ReturnType, RwLock, RwLockReadGuard,
    Slice, Step_Continue, TreeWriter,
};

#[derive(Debug)]
pub struct AuthorData {
    pub name: Vec<u8>,
    pub pos: usize,
    pub length: usize,
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

#[derive(Debug)]
pub struct Title {
    pub inner: RwLock<TitleData>,
    pub index: usize,
}

impl Title {
    pub fn new(index: usize) -> Self {
        Self {
            inner: Default::default(),
            index,
        }
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

            //  if co.is_debug() {
            //     crate::ghidra_marker!("rsi");
            // } else {
            //     crate::ghidra_marker!("esi");
            // }

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

            // if co.is_debug() {
            //     crate::ghidra_marker!("rdi");
            // } else {
            //     crate::ghidra_marker!("edi");
            // }

            curr_slice = rest;
        }
    }

    async fn parse_authors(&self, co: &impl Context, mut curr_slice: Slice<'_>) {
        let mut import_finder = ImportFinder::new(Import::get_all());

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
                sep = b"";
                let old_author = mem::replace(
                    &mut author_data,
                    AuthorData {
                        name: Vec::new(),
                        pos: 0,
                        length: 0,
                    },
                );
                self.add_author(
                    co,
                    &mut import_finder,
                    &mut parsed_first,
                    old_author,
                    slice.pos,
                )
                .await;

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
        self.add_author(
            co,
            &mut import_finder,
            &mut parsed_first,
            author_data,
            curr_slice.end(),
        )
        .await;

        let mut this = self.inner.write();
        this.by_section_length = curr_slice.end() - this.title_length;
    }

    async fn add_author(
        &self,
        co: &impl Context,
        import_finder: &mut ImportFinder,
        parsed_first: &mut bool,
        author_data: AuthorData,
        curr_pos: usize,
    ) {
        if author_data.name.len() > 0 {
            {
                let mut this = self.inner.write();
                this.authors.push(author_data);
                this.by_section_length = curr_pos - this.title_length;
            }
            Step_Continue!(co, self, curr_pos, "{} parsed an author");
            if *parsed_first {
                {
                    let mut this = self.inner.write();
                    let last_author = this.authors.last().unwrap();
                    let name_slice = Slice::from(last_author.name.as_slice(), last_author.pos);
                    let import_vec = import_finder.find(name_slice);
                    this.imports.extend(import_vec);
                }
                Step_Continue!(co, self, curr_pos, "{} parsed imports for author");
            }
            *parsed_first = true;
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

impl Paragraph for Title {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl TreeWriter for Title {
    fn write_lisp(&self) -> String {
        let this = self.inner.read();
        // escape potetial " in title
        let title_str = str::from_utf8(&this.title).unwrap().replace("\"", "\\\"");
        let title_length = this.title_length;
        let by_section_length = this.by_section_length;

        if by_section_length == 0 {
            format!("(title \"{title_str}\"$${title_length})")
        } else {
            let authors_str = this.authors.iter().fold(String::new(), |acc, data| {
                let author_str = str::from_utf8(&data.name).unwrap();
                format!("{acc} \"{author_str}\"@{}$${}", data.pos, data.length)
            });

            let imports_str = this.imports.iter().fold(String::new(), |acc, data| {
                let import_str = data.import.name();
                format!("{acc} \"{import_str}\"@{}$${}", data.pos, data.length)
            });
            format!(
            "(title \"{title_str}\" (by${title_length}$${by_section_length} (authors{authors_str}) (imports{imports_str})))",
            )
        }
    }

    fn write_lint(&self, writer: &mut LintWriter, _indent: u8) {
        let this = self.inner.read();

        fn write_authors(writer: &mut LintWriter, this: &RwLockReadGuard<TitleData>) {
            let mut authors = this.authors.iter().peekable();
            let mut imports = this.imports.iter().peekable();
            while let Some(author) = authors.peek() {
                writer.write_up_to_as(LintColor::TitleSeparator, author.pos);
                let author_end = author.pos + author.length;
                while let Some(import) = imports.peek() {
                    // if import before end -- write that
                    if import.pos < author_end {
                        writer.write_up_to_as(LintColor::TitleAuthor, import.pos);
                        writer.write_as(LintColor::TitleImport, import.length.into());
                        imports.next();
                    } else {
                        break;
                    }
                }
                writer.write_up_to_as(LintColor::TitleAuthor, author_end);
                authors.next();
            }
        }

        writer.write_up_to_as(LintColor::Title, this.title_length);
        if this.by_section_length > 0 {
            writer.write_as(LintColor::TitleBy, 2);
            write_authors(writer, &this);
            writer.write_up_to_as(
                LintColor::TitleSeparator,
                this.title_length + this.by_section_length,
            );
        }
    }

    fn write_javascript(&self, _indent: Indent) -> String {
        let this = self.inner.read();
        let title_str = str::from_utf8(&this.title).unwrap();
        let mut authors = this
            .authors
            .iter()
            .map(|e| str::from_utf8(&e.name).unwrap());

        let author_count = authors.len();

        let primary_author_str = {
            authors.next().map_or("".to_string(), |name| {
                format!("\n// Primary Author: {name}")
            })
        };

        let secondary_authors_str = {
            // let secondary_authors = authors.collect::<Vec<_>>();
            match author_count {
                0 | 1 => "".to_string(),
                _ => format!(
                    "\n// Secondary Author{}: {}",
                    if author_count == 2 { "s" } else { "" },
                    authors.join(", ")
                ),
            }
        };

        let imports_str = {
            let mut imports = this.imports.iter().map(|e| e.import.name());
            match imports.len() {
                0 => "".to_string(),
                len => format!(
                    "\n// Import{}: {}",
                    if len > 1 { "s" } else { "" },
                    imports.join(", ")
                ),
            }
        };

        format!("// Title: {title_str}{primary_author_str}{secondary_authors_str}{imports_str}",)
    }
}

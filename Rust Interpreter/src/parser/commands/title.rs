use std::{any::Any, fmt::format, mem};

use bstr::{ByteSlice, ByteVec};
use itertools::Itertools;
use std::str;

use crate::parser::{context::ParseResult, parsable_vec::ParsableVec};

use super::{
    close_data, CloseData, Context, FailReason, Import, ImportData, ImportFinder, Indent,
    LintColor, LintWriter, Paragraph, Parsable, ParseTreeObj, ReturnType, RwLock, RwLockReadGuard,
    Slice, TreeWriter,
};

#[derive(Debug)]
pub struct AuthorData {
    pub name: Vec<u8>,
    pub pos: usize,
    pub length: usize,
}

#[derive(Debug, Default)]
pub struct Title {
    /// the poem title
    pub title: Vec<u8>,
    /// title is trimmed
    pub title_length: usize,
    /// the author names
    pub authors: Vec<AuthorData>,
    // the imports: (type, position, length)
    pub imports: Vec<ImportData>,
    //
    pub author_section_length: usize,

    pub index: usize,
}

struct AuthorStepState {
    import_finder: ImportFinder,
    parsed_first: bool,
    sep: &'static [u8],
    author_data: AuthorData,
}

impl Title {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }

    pub fn parse(&mut self, co: Context, slice: Slice) -> ParseResult {
        self.find_title(co, slice, b"")
    }

    /// adds title looking at lines until a line starts with "by "
    fn find_title(&mut self, co: Context, curr_slice: Slice, space: &[u8]) -> ParseResult {
        let (title, rest) = curr_slice.get_next_line();

        if co.is_debug() {
            crate::ghidra_marker!("rsi");
        } else {
            crate::ghidra_marker!("esi");
        }

        let is_first_line = space.len() == 0;

        // println!(
        //     "{} - {:?}- {}",
        //     is_first_line,
        //     title.str[..3].to_ascii_lowercase(),
        //     title.len() 
        // );

        if title.len() == 0 {
            co.result_cont::<Self>(
                move |_this, co, slice| co.result_match(slice.len(), ReturnType::Null),
                title,
                format!("Title never found keyword \"by\""),
            )
        } else if !is_first_line && title.str[..3].to_ascii_lowercase() == b"by " {
            self.author_section_length = 2;
            println!("parsed title");
            co.result_cont::<Self>(
                Title::parse_authors,
                curr_slice.offset(3),
                format!("Title found author section with the keyword \"by\""),
            )
        } else {
            self.title.push_str(title.str.trim());
            self.title.push_str(space);
            self.title_length += title.end();

            let description = if is_first_line {
                "Title added first line to title"
            } else {
                "Title did not find keyword \"by\" so added line to title"
            };

            co.result_cont::<Self>(
                move |this, co, slice| this.find_title(co, slice, b"\n"),
                rest,
                description.to_string(),
            )
        }
    }

    fn parse_authors(&mut self, co: Context, slice: Slice) -> ParseResult {
        let author_state = AuthorStepState {
            import_finder: ImportFinder::new(Import::get_all()),
            parsed_first: false,
            sep: b"",
            author_data: AuthorData {
                name: Vec::new(),
                pos: slice.pos,
                length: 0,
            },
        };

        self.find_authors_length_check(co, slice, author_state)
    }

    // this exists because find_authors is called by itself and the last author needs to be added if length is 0 in that case
    fn find_authors_length_check(
        &mut self,
        co: Context,
        slice: Slice,
        authors_state: AuthorStepState,
    ) -> ParseResult {
        self.author_section_length = slice.pos - self.title_length;
        if slice.len() == 0 {
            co.result_match(slice.end(), ReturnType::Null)
        } else {
            self.find_authors(co, slice, authors_state)
        }
    }

    fn find_authors(
        &mut self,
        co: Context,
        slice: Slice,
        mut authors_state: AuthorStepState,
    ) -> ParseResult {
        // update length
        self.author_section_length = slice.pos - self.title_length;

        // if slice not empty
        //
        if slice.len() > 0 {
            let (word, rest) = slice.get_next_slice();

            // if word is separator
            // add author
            if Self::is_separator(word.str).close_count > 0 {
                authors_state.sep = b"";
                self.add_author(co, rest, authors_state)

            // if word is part of author name
            // edit current author and loop
            } else {
                let author_data = &mut authors_state.author_data;
                if author_data.pos == 0 {
                    author_data.pos = word.pos;
                }
                author_data.name.push_str(authors_state.sep);
                author_data.name.push_str(word.str);
                author_data.length = word.end() - author_data.pos;
                authors_state.sep = b" ";

                co.result_cont::<Self>(
                    move |this, co, slice| this.find_authors(co, slice, authors_state),
                    rest,
                    format!("Title found an author name"),
                )
            }

        // slice is empty
        // add last author if needed
        } else {
            self.add_author(co, slice, authors_state)
        }
    }

    fn add_author(
        &mut self,
        co: Context,
        rest: Slice,
        mut authors_state: AuthorStepState,
    ) -> ParseResult {
        if authors_state.author_data.name.len() > 0 {
            let author_data = mem::replace(
                &mut authors_state.author_data,
                AuthorData {
                    name: Vec::new(),
                    pos: 0,
                    length: 0,
                },
            );
            self.authors.push(author_data);
        }
        co.result_cont::<Self>(
            move |this, co, slice| this.add_imports(co, slice, authors_state),
            rest,
            format!("Title found a separator"),
        )
    }

    fn add_imports(
        &mut self,
        co: Context,
        slice: Slice,
        mut authors_state: AuthorStepState,
    ) -> ParseResult {
        let description = if authors_state.parsed_first {
            // let mut this = self;
            let last_author = self.authors.last().unwrap();
            let name_slice = Slice::from_buf(last_author.name.as_slice(), last_author.pos);
            let import_vec = authors_state.import_finder.find(name_slice);
            self.imports.extend(import_vec);
            "Title found imports in author"
        } else {
            authors_state.parsed_first = true;
            "Title passed imports for first author"
        };

        co.result_cont::<Self>(
            move |this, co, slice| this.find_authors_length_check(co, slice, authors_state),
            slice,
            description.to_string(),
        )
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Parsable for Title {
    fn get_children(&self) -> Vec<usize> {
        Vec::new()
    }
}

impl Paragraph for Title {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl TreeWriter for Title {
    fn write_lisp(&self, _vec: &ParsableVec) -> String {
        // escape potetial " in title
        let title_str = str::from_utf8(&self.title).unwrap().replace("\"", "\\\"");
        let title_length = self.title_length;
        let author_section_length = self.author_section_length;
        let index = self.index;

        if author_section_length == 0 {
            format!("(title \"{title_str}\"$${title_length})")
        } else {
            let authors_str = self.authors.iter().fold(String::new(), |acc, data| {
                let author_str = str::from_utf8(&data.name).unwrap();
                format!("{acc} \"{author_str}\"@{}$${}", data.pos, data.length)
            });

            let imports_str = self.imports.iter().fold(String::new(), |acc, data| {
                let import_str = data.import.name();
                format!("{acc} \"{import_str}\"@{}$${}", data.pos, data.length)
            });
            format!(
            "(title:{index} \"{title_str}\" (by${title_length}$${author_section_length} (authors{authors_str}) (imports{imports_str})))",
            )
        }
    }

    fn write_lint(&self, _vec: &ParsableVec, writer: &mut LintWriter, _indent: u8) {
        fn write_authors(writer: &mut LintWriter, this: &Title) {
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

        writer.write_up_to_as(LintColor::Title, self.title_length);
        if self.author_section_length > 0 {
            writer.write_as(LintColor::TitleBy, 2);
            write_authors(writer, &self);
            writer.write_up_to_as(
                LintColor::TitleSeparator,
                self.title_length + self.author_section_length,
            );
        }
    }

    fn write_javascript(&self, _vec: &ParsableVec, _indent: Indent) -> String {
        let title_str = str::from_utf8(&self.title).unwrap();
        let mut authors = self
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
            let mut imports = self.imports.iter().map(|e| e.import.name());
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

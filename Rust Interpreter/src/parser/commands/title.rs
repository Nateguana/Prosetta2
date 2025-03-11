use bstr::{ByteSlice, ByteVec};

use super::{close_data, CloseData, Command, Context, FailReason, Import, ReturnType, Slice};

pub struct AuthorData {
    pub name: Vec<u8>,
    pub pos: usize,
    pub length: usize,
}

pub struct ImportData {
    pub name: Import,
    pub pos: usize,
    pub length: u8,
}

/// The command for Title
///
#[derive(Default)]
pub struct Title {
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

impl Title {
    ///add title data and returns slice after by
    async fn find_title<'a>(&mut self, co: &Context, slice: Slice<'a>) -> Slice<'a> {
        let mut curr_slice = slice;
        let mut space: &[u8] = b"";
        loop {
            let (title, rest) = curr_slice.get_next_line();

            // add title
            self.title.push_str(title.str.trim());
            self.title.push_str(space);
            self.title_length = rest.pos;
            space = b"\n";

            co.step_move(self, rest.pos).await;

            // no more text
            if rest.len() > 0 {
                return rest;
            }

            // find "by"
            if let Some((word, rest2)) = rest.get_next_word_arg() {
                if word.str.to_ascii_lowercase() == b"by" {
                    return rest2;
                }
            }

            curr_slice = rest;
        }
    }

    async fn parse_authors(&mut self, _co: &Context, _slice: Slice<'_>) {
        // let mut parsed_first = false;
        // let mut curr_slice = slice;
        // loop {
        //     let (title, rest) = curr_slice.get_next_slice();
        //     co.step_move(&self, pos).await;
        // }
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
#[async_trait::async_trait]
impl Command for Title {
    fn new() -> Self {
        Default::default()
    }
    async fn try_parse(
        &mut self,
        co: &Context,
        slice: Slice<'_>,
    ) -> Result<(usize, ReturnType), FailReason> {
        let curr_slice = self.find_title(co, slice).await;
        self.parse_authors(co, curr_slice).await;
        Ok((slice.end(), ReturnType::Null))
    }
    fn name(&self) -> &'static str {
        "Title"
    }
    // fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
    //     if self.parsing_names {
    //         // names cannot be more than 255
    //         if word.len() > 255 {
    //             return MatchResult::Continue(0);
    //         } else if word.len() == 0 {
    //             let title = mem::replace(&mut self.data, Title::new());
    //             *env.expr = Expr::Title { data: title };
    //             return MatchResult::Matched(word.pos, ReturnType::Void, false);
    //         }
    //         let separator = Self::is_separator(word.str);
    //         // is name
    //         if separator.close_count == 0 {
    //             if self.is_author_closed {
    //                 // first author already done
    //                 if self.data.authors.len() >= 1 {
    //                     self.add_imports(env, word.str, word.pos);
    //                 }
    //                 self.data
    //                     .authors
    //                     .push((word.str.to_vec(), word.pos, word.str.len()));
    //             } else {
    //                 // will always exist
    //                 let author = self.data.authors.last_mut().unwrap();
    //                 author.0.push(b' ');
    //                 author.0.extend_from_slice(&word.str);
    //                 author.2 = word.end() - author.1;
    //                 // second author started
    //                 if self.data.authors.len() >= 2 {
    //                     self.add_imports(env, word.str, word.pos);
    //                 }
    //             }
    //             self.is_author_closed = false;
    //             MatchResult::Continue(0)
    //             // is name close
    //         } else {
    //             self.is_author_closed = true;
    //             self.data.delim.push((word.pos, separator.close_length));

    //             MatchResult::Continue(0)
    //         }
    //     } else if word.len() >= 2 && word.str.to_ascii_lowercase() == b"by" {
    //         self.data
    //             .title
    //             .extend_from_slice(&env.full_text[..word.pos].trim());
    //         self.data.by_start = word.pos;
    //         self.parsing_names = true;

    //         MatchResult::Continue(0)
    //     } else {
    //         let slice = find_newline(&rest, 0);
    //         if let Some(newline) = slice {
    //             let offset = newline.pos - rest.pos;
    //             MatchResult::Continue(offset)
    //         // did not find another new line -- poem has ended -- will never match
    //         } else {
    //             MatchResult::Failed
    //         }
    //     }
    // }

    // fn step_match(
    //     &mut self,
    //     _env: &mut Environment,
    //     _child_index: Option<(usize, ReturnType)>,
    //     _word: &Slice,
    //     _rest: &Slice,
    // ) -> MatchResult {
    //     // has no child to match - fn should never be called
    //     unreachable!()
    // }

    // fn get_name(&self) -> &'static str {
    //     "Title"
    // }

    // fn get_type(&self) -> StateType {
    //     StateType::None
    // }
}

// impl TitleState {
//     pub fn new() -> Self {
//         Self {
//             parsing_names: false,
//             is_author_closed: true,
//             data: Title::new(),
//         }
//     }

//     ///returns and optinal length of the close
//     fn is_separator(str: &[u8]) -> CloseData {
//         if str.len() >= 3 && str == b"and" {
//             CloseData {
//                 close_count: 1,
//                 close_length: 3,
//                 only_forced: true,
//             }
//         } else if str.len() >= 1 && str == b"&" {
//             CloseData {
//                 close_count: 1,
//                 close_length: 1,
//                 only_forced: true,
//             }
//         } else {
//             get_close_data(str)
//         }
//     }
//     fn add_imports(&mut self, env: &mut Environment, name: &[u8], index: usize) {
//         let lower_name = name.to_ascii_lowercase();
//         let imports = Import::get_all();
//         if let Some((offset, _, imp_index)) =
//             parser_structs::try_get_best_val(&lower_name, &mut imports.iter().map(|e| e.1), &|_| {
//                 true
//             })
//         {
//             env.trigger_word_data.add_val(
//                 offset as usize + index,
//                 offset as usize + index + imports[imp_index].1.len() as usize,
//                 alias::WordTriggerType::Import(imports[imp_index].1.to_vec()),
//             );
//             self.data.imports.push((
//                 imports[imp_index].0,
//                 offset as usize + index,
//                 imports[imp_index].1.len() as u8,
//             ))
//         }
//     }
// }

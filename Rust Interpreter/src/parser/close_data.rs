use super::slice::Slice;

///the chars that are counted as being part of words
const OTHER_CHARS: &[u8] = b"-+^/'";
///can the char be part of a word
fn is_valid_word_char(char: u8) -> bool {
    char.is_ascii_alphanumeric() || OTHER_CHARS.contains(&char)
}

///chars that close functions
const END_CHARS: &[u8] = b".?!,:";
///can the char close a command
fn is_valid_close_char(char: u8) -> bool {
    END_CHARS.contains(&char)
}

///the chars that are returned single but are not closes
const NON_CLOSE_CHARS: &[u8] = b"\"&[]";
///shoudl the char be made into a 1 len slice
pub fn is_non_close_but_still_single(char: u8) -> bool {
    NON_CLOSE_CHARS.contains(&char)
}

/// does slice consist of a closing character
pub fn is_close(slice: &Slice) -> bool {
    // does str close something
    get_close_data(slice.str).close_length != 0
}

/// For when a close is forced rather than able.
pub fn is_mandatory_close(slice: &Slice) -> bool {
    let cd = get_close_data(slice.str);
    cd.close_length != 0 && !cd.only_forced
}

pub struct CloseData {
    pub close_count: u8,
    pub close_length: u8,
    pub only_forced: bool,
}

fn is_word_stopper(line: &[u8]) -> bool {
    line.len() >= 2 && &line[..2] == b"--"
}

/// gets the number of times the characters at line[index] should be repeated and the offset after
/// returns (repeat_count,offset)
pub fn get_close_data(line: &[u8]) -> CloseData {
    if line.len() >= 3 && &line[..3] == b"..." {
        CloseData {
            close_count: 10,
            close_length: 3,
            only_forced: false,
        }
    } else if line.len() >= 3 && &line[..3] == b"---" {
        CloseData {
            close_count: 3,
            close_length: 3,
            only_forced: false,
        }
    } else if is_word_stopper(line) {
        CloseData {
            close_count: 2,
            close_length: 2,
            only_forced: false,
        }
    } else if line.len() >= 1 {
        match line[0] {
            b'.' | b':' => CloseData {
                close_count: 1,
                close_length: 1,
                only_forced: false,
            },
            b',' | b';' => CloseData {
                close_count: 1,
                close_length: 1,
                only_forced: true,
            },
            b'?' | b'!' => CloseData {
                close_count: 2,
                close_length: 1,
                only_forced: false,
            },
            _ => CloseData {
                close_count: 0,
                close_length: 0,
                only_forced: false,
            },
        }
    } else {
        CloseData {
            close_count: 0,
            close_length: 0,
            only_forced: false,
        }
    }
}

///gets the next slice. a slice consists of either a word or a closing character
pub fn get_next_slice<'a>(slice: &Slice<'a>, mut start: usize) -> (Slice<'a>, Slice<'a>) {
    // find start of word
    start = start.min(slice.len());
    while start < slice.len()
        && !is_valid_word_char(slice.str[start])
        && !is_valid_close_char(slice.str[start])
        && !is_non_close_but_still_single(slice.str[start])
    {
        start += 1;
    }

    // find end of word
    let mut end = start;

    let close_data = get_close_data(&slice.str[start..]);
    if close_data.close_length != 0 {
        end += close_data.close_length as usize;
    } else if end < slice.len() && is_non_close_but_still_single(slice.str[start]) {
        end += 1;
    } else {
        while end < slice.len()
            && is_valid_word_char(slice.str[end])
            && !is_word_stopper(&slice.str[end..])
        {
            end += 1;
        }
    }

    (
        Slice {
            str: &slice.str[start..end],
            pos: slice.pos + start,
        },
        Slice {
            str: &slice.str[end..],
            pos: slice.pos + end,
        },
    )
}

/// returns the rest after the end of the word
pub fn find_word_end<'a>(slice: &'a Slice<'a>, start: usize) -> Slice<'a> {
    // find end of word

    let mut end = start.min(slice.len());
    while end < slice.len() && is_valid_word_char(slice.str[end]) {
        end += 1;
    }
    //let test = end < slice.len();
    //end = end.min(slice.len());
    Slice {
        str: &slice.str[end..],
        pos: slice.pos + end,
    }
}

/// returns (close, rest) after finding close
pub fn find_close_slice<'a>(slice: &'a Slice<'a>, mut start: usize) -> Option<(Slice<'a>, Slice<'a>)> {
    // find end char
    let mut close_len = 0;
    while start < slice.len() {
        close_len = get_close_data(&slice.str[start..]).close_length;
        if close_len == 0 {
            start += 1;
        } else {
            break;
        }
    }
    if start < slice.len() {
        // find end of period
        let end = start + close_len as usize;
        Some((
            Slice {
                str: &slice.str[start..end],
                pos: slice.pos + start,
            },
            Slice {
                str: &slice.str[end..],
                pos: slice.pos + end,
            },
        ))
    } else {
        None
    }
}

/// returns the rest after finding the next closing character
pub fn find_close<'a>(slice: &'a Slice<'a>, start: usize) -> Option<Slice<'a>> {
    find_close_slice(slice, start).map(|s| s.1)
}

const INVALID_SYMBOL_CHARS: &[u8] = b"\'-";
///is char a valid char in a symbol
fn is_symbol_char(char: u8) -> bool {
    !INVALID_SYMBOL_CHARS.contains(&char)
}
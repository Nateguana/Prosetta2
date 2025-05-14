use std::{collections::VecDeque, mem};

use super::{commands::AliasName, imports::Import, slice::Slice};

#[derive(Clone)]
struct AliasParseData {
    alias: AliasName,
    index: u8,
}

struct AliasFinder {
    arr: [VecDeque<AliasParseData>; 26],
}

impl AliasFinder {
    pub fn new(arr: [VecDeque<AliasParseData>; 26]) -> Self {
        Self { arr: arr.clone() }
    }

    fn add_letter(&mut self, letter: u8) -> Vec<AliasName> {
        let mut ret = Vec::new();
        let array = self.get_array(letter);
        if let Some(array) = array {
            let list = mem::take(array);
            for mut alias in list {
                alias.index += 1;
                if alias.index == 2 {
                    ret.push(alias.alias)
                } else {
                    let array = self.get_array(alias.alias[alias.index as usize]).unwrap();
                    match alias.index {
                        1 => array.push_front(alias),
                        0 => array.push_back(alias),
                        _ => unreachable!(),
                    }
                }
            }
        }
        return ret;
    }

    fn get_array(&mut self, letter: u8) -> Option<&mut VecDeque<AliasParseData>> {
        self.arr.get_mut((letter - b'a') as usize)
    }
}

#[derive(Clone)]
pub struct ImportParseData {
    alias: AliasName,
    import: Import,
    index: u8,
}

impl ImportParseData {
    pub const fn new(import: Import, alias: AliasName) -> Self {
        Self {
            alias,
            import,
            index: 0,
        }
    }
}

struct ImportFinder {
    arr: Vec<ImportParseData>,
}

impl ImportFinder {
    pub fn new(arr: &[ImportParseData]) -> Self {
        Self { arr: arr.to_vec() }
    }

    /// find imports
    pub fn find(&mut self, slice: Slice) -> Vec<(usize, Import)> {
        let mut ret = Vec::new();

        for j in 0..slice.len() {
            let char = slice.str[j];
            if char != b'\'' {
                let mut index = 0;
                while index < self.arr.len() {
                    let alias = &mut self.arr[index];
                    //
                    if alias.alias[alias.index as usize] == char {
                        alias.index += 1;

                        // import has finished
                        if alias.index >= 3 {
                            let element = (slice.pos + j, self.arr.swap_remove(index).import);
                            ret.push(element);

                            // imports cannot intersect
                            for ele in &mut self.arr {
                                ele.index = 0;
                            }

                            // just removed element -- try index again
                            index -= 1;
                        }
                    } else {
                        alias.index = 0;
                    }
                    index += 1;
                }
            }
        }

        ret
    }
}

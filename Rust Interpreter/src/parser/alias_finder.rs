use std::{collections::VecDeque, mem};

use super::{
    commands::AliasName,
    imports::{Import, ImportData, ImportParseData},
    slice::Slice,
};

#[derive(Clone)]
pub struct AliasParseData {
    alias: AliasName,
    index: u8,
}

#[derive(Clone)]
pub struct AliasFinder {
    arr: Box<[VecDeque<AliasParseData>; 26]>,
}

impl AliasFinder {
    pub fn new() -> Self {
        Self {
            arr: Default::default(),
        }
    }

    pub fn add(&mut self, alias: AliasName) {
        let array = self.get_array(alias[0]).unwrap();
        array.push_back(AliasParseData { alias, index: 0 });
    }

    fn add_letter(&mut self, letter: u8) -> Vec<AliasName> {
        let mut ret = Vec::new();
        let array = self.get_array(letter);
        if let Some(array) = array {
            let list = mem::take(array);
            for mut alias in list {
                alias.index += 1;
                if alias.index == 3 {
                    ret.push(alias.alias)
                } else {
                    let array = self.get_array(alias.alias[alias.index as usize]).unwrap();
                    match alias.index {
                        2 => array.push_front(alias),
                        1 => array.push_back(alias),
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

pub struct ImportFinder {
    arr: Vec<ImportParseData>,
}

impl ImportFinder {
    pub fn new(arr: &[ImportParseData]) -> Self {
        Self { arr: arr.to_vec() }
    }

    /// find imports
    pub fn find(&mut self, slice: Slice) -> Vec<ImportData> {
        let mut ret = Vec::new();

        for j in 0..slice.len() {
            let char = slice.str[j].to_ascii_lowercase();
            if char != b'\'' {
                let mut index = 0;
                while index < self.arr.len() {
                    let alias = &mut self.arr[index];
                    // check if letter matches
                    if alias.alias[alias.index as usize] == char {
                        alias.index += 1;

                        // import has finished
                        if alias.index >= 3 {
                            let import_parse_data = self.arr.swap_remove(index);
                            let element = ImportData {
                                pos: slice.pos + j - 2,
                                alias: import_parse_data.alias,
                                import: import_parse_data.import,
                                length: 3,
                            };

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

use std::{cmp::Ordering, collections::VecDeque, mem, sync::Arc};

#[path = "../tests/alias_finder.rs"]
mod tests;

use super::{
    commands::AliasName,
    imports::{Import, ImportData, ImportParseData},
    slice::Slice,
};

#[derive(Clone, Debug)]
#[repr(align(2))]
pub struct AliasParseData {
    alias: AliasName,
    index: u8,
    pos: [u8; 3],
}

pub struct AliasFinderArray {
    arr: [Vec<AliasParseData>; 26],
}

impl AliasFinderArray {
    pub fn new() -> Self {
        Self {
            arr: [const { Vec::new() }; 26],
        }
    }
    pub fn add(&mut self, alias: AliasName) {
        let array = self.get_mut(alias[0]).unwrap();
        array.push(AliasParseData {
            alias,
            index: 0,
            pos: [0; 3],
        });
    }

    fn get_mut(&mut self, letter: u8) -> Option<&mut Vec<AliasParseData>> {
        self.arr.get_mut((letter - b'a') as usize)
    }

    fn get(&self, letter: u8) -> Option<&Vec<AliasParseData>> {
        self.arr.get((letter - b'a') as usize)
    }
}

// #[derive(Clone)]
pub struct AliasFinder {
    base: Arc<AliasFinderArray>,
    inner: Box<AliasFinderArray>,
    index: u8,
}

impl AliasFinder {
    pub fn new(base: Arc<AliasFinderArray>) -> Self {
        Self {
            base,
            inner: Box::new(AliasFinderArray::new()),
            index: 0,
        }
    }

    fn add_letter(&mut self, letter: u8) -> Vec<AliasParseData> {
        let mut ret = Vec::new();
        let my_array = self.inner.get_mut(letter);
        if let Some(array) = my_array {
            let list = mem::take(array);
            for mut alias in list {
                alias.pos[alias.index as usize] = self.index;
                alias.index += 1;
                if alias.index == 3 {
                    ret.push(alias)
                } else {
                    let array = self
                        .inner
                        .get_mut(alias.alias[alias.index as usize])
                        .unwrap();
                    array.push(alias);
                }
            }

            for mut alias in self.base.get(letter).unwrap().into_iter().cloned() {
                alias.pos[0] = self.index;
                alias.index += 1;
                let array = self
                    .inner
                    .get_mut(alias.alias[alias.index as usize])
                    .unwrap();
                array.push(alias);
            }
        }
        ret.sort_by(Self::compare_alias_parse);
        self.index += 1;
        return ret;
    }

    fn compare_alias_parse(a: &AliasParseData, b: &AliasParseData) -> Ordering {
        fn calc(a: &AliasParseData) -> u16 {
            (a.pos[0] as u16) << 8 | (!a.pos[1] as u16)
        }
        dbg!(calc(dbg!(b))).cmp(&calc(a))
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

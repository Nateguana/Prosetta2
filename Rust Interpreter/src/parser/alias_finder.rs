#[path = "../tests/alias_finder.rs"]
mod tests;

use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    mem,
    sync::Arc,
};

use ufmt::derive;

use super::{
    commands::AliasName,
    imports::{Import, ImportData, ImportParseData},
    slice::Slice,
    tree_writer::lint_writer::{LintColor, LintWriter},
    types::{ReturnType, ReturnTypeSet},
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

#[derive(Debug)]
pub struct AliasLoc {
    index: usize,
    pos: [u8; 2],
}

impl AliasLoc {
    pub fn write_lisp(&self) -> String {
        format!(
            "@{},{},{}",
            self.index,
            self.index + self.pos[0] as usize,
            self.index + self.pos[1] as usize
        )
    }
    pub fn write_lint(&self, writer: &mut LintWriter, indent: u8) {
        let color =
            [LintColor::Alias1, LintColor::Alias2, LintColor::Alias3][(indent % 3) as usize];
        writer.write_up_to(self.index);
        writer.write_as(color, 1);
        for pos in self.pos {
            writer.write_up_to(pos as usize);
            writer.write_as(color, 1);
        }
    }
}

pub struct ParsedAliasData {
    index: usize,
    pos: [u8; 2],
    alias: AliasName,
}

impl ParsedAliasData {
    fn from(data: AliasParseData, start: usize) -> Self {
        Self {
            index: start + data.pos[0] as usize,
            pos: [data.pos[1], data.pos[2]],
            alias: data.alias,
        }
    }
    pub fn get_loc(&self) -> AliasLoc {
        AliasLoc {
            index: self.index,
            pos: self.pos,
        }
    }
}

pub struct AliasFinder {
    base: Arc<AliasFinderArray>,
    inner: Box<AliasFinderArray>,
    used_set: HashSet<AliasName>,
    index: u8,
}

impl AliasFinder {
    pub fn new(base: Arc<AliasFinderArray>) -> Self {
        Self {
            base,
            inner: Box::new(AliasFinderArray::new()),
            used_set: HashSet::new(),
            index: 0,
        }
    }

    pub fn find(&mut self, slice: Slice) -> Vec<ParsedAliasData> {
        let mut ret = Vec::new();

        if slice.len() <= 200 {
            for index in 0..slice.len() {
                ret.extend(
                    self.add_letter(slice.str[index])
                        .into_iter()
                        .map(|x| ParsedAliasData::from(x, slice.pos)),
                );
            }
        }
        ret
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

        // sort aliases by pos
        ret.sort_by(Self::compare_alias_parse);

        // add to hashset and remove dublicates
        ret.retain(|data| self.used_set.insert(data.alias));

        self.index = self.index.checked_add(1).unwrap();
        return ret;
    }

    fn compare_alias_parse(a: &AliasParseData, b: &AliasParseData) -> Ordering {
        // calculate score via position math magic
        fn calc(a: &AliasParseData) -> u16 {
            (a.pos[0] as u16) << 8 | (!a.pos[1] as u16)
        }
        calc(b).cmp(&calc(a))
    }
}

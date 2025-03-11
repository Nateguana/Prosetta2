use std::{collections::VecDeque, mem};

use super::AliasName;

struct NoneExpr{

}

struct NoneStat{
    
}

struct AliasWordData {
    alias: AliasName,
    index: u8,
}
struct AliasFinder {
    arr: [VecDeque<AliasWordData>; 26],
}

impl AliasFinder {
    pub fn new(aliases: &[AliasName]) -> Self {
        let mut this = Self {
            arr: Default::default(),
        };
        for &alias in aliases {
            this.get_array(alias[0]).push_back(AliasWordData { alias, index: 0 });
        }
        this
    }
    pub fn add_letter(&mut self, letter: u8) -> Vec<AliasName> {
        let mut ret = Vec::new();
        let list  = mem::take( self.get_array(letter));
        for alias in list{
            match alias.index{
                2=>ret.push(alias.alias),
                1=>self.get_array(letter).push_front(alias),
                0=>self.get_array(letter).push_back(alias),
                _=>unreachable!()
            }
        }
        return ret;
    }
    fn get_array(&mut self, letter:u8)->&mut VecDeque<AliasWordData>{
        &mut self.arr[(letter - b'a') as usize]
    }   
}

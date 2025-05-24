use std::collections::{HashMap, VecDeque};

use super::{
    alias_finder::AliasParseData,
    commands::{self as comm, AliasName, Aliased},
    imports::{Import, ImportData},
};

// fn test() {
//     let g = vec![];
// }

macro_rules! make_alias_data {
    ($e:ty) => { $crate::parser::alias_data::AliasData::new(<$e>::alias(),||Box::new(<$e>::new())) };
    ($e:ty, $($es:ty),+) => { crate::parser::alias_data::AliasData::append(make_alias_data!($($es),*),$e::alias(),$e::new) };
}

// struct

struct AliasDataBuilder {
    map: Vec<(AliasName, fn() -> Box<dyn Aliased>)>,
}

impl AliasDataBuilder {
    fn new(name: AliasName, func: fn() -> Box<dyn Aliased>) -> Self {
        Self {
            map: vec![(name, func)],
        }
    }

    fn append(mut self: Self, name: AliasName, func: fn() -> Box<dyn Aliased>) -> Self {
        self.map.push((name, func));

        self
    }
}

type AliasMap = HashMap<AliasName, fn() -> Box<dyn Aliased>>;
struct AliasData {
    expr_map: AliasMap,
    expr_data: Box<[VecDeque<AliasParseData>; 26]>,
    stat_map: AliasMap,
    stat_data: Box<[VecDeque<AliasParseData>; 26]>,
}

impl AliasData {
    // pub fn new(imports: &[Import]) -> Self {
    //     let this = Self {
    //         expr_map: HashMap::new(),
    //         stat_map: HashMap::new(),
    //     };

    //     this
    // }
    // fn add_expr_aliases(&mut self, builder: AliasDataBuilder) {
    //     for (alias, func) in builder.map {
    //         self.expr_map.insert(alias, func);
    //         self.expr_data[alias[0]]
    //     }
    // }
}
impl AliasData {
    fn add_base_aliases() {}
}

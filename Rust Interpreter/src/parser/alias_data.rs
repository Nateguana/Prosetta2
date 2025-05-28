use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use super::{
    alias_finder::{AliasFinderArray, AliasLoc, AliasParseData},
    commands::{self as comm, addition::Addition, stroke::Stroke, AliasName, Aliased},
    imports::{Import, ImportData},
};

// fn test() {
//     let g = vec![];
// }

macro_rules! make_alias_data {
    () => { AliasDataBuilder::new() };
    // ($e:ty) => { AliasDataBuilder::append(AliasDataBuilder::new(<$e>::alias(),||Box::new(<$e>::new())) };
    ($e:ty $(,$es:ty)*) => { AliasDataBuilder::append(make_alias_data!($($es,)*),<$e>::alias(),|loc|Box::new(<$e>::new(loc))) };
}

// struct
type AliasNewFunc = fn(AliasLoc) -> Box<dyn Aliased>;
type AliasMap = HashMap<AliasName, AliasNewFunc>;

struct AliasDataBuilder {
    map: Vec<(AliasName, AliasNewFunc)>,
}

impl AliasDataBuilder {
    fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn append(mut self: Self, name: AliasName, func: AliasNewFunc) -> Self {
        self.map.push((name, func));

        self
    }
}

struct AliasData {
    expr_map: AliasMap,
    stat_map: AliasMap,
    expr_data: Arc<AliasFinderArray>,
    stat_data: Arc<AliasFinderArray>,
}

impl AliasData {
    pub fn new(imports: &[Import]) -> Self {
        let mut expr_data = (AliasMap::new(), AliasFinderArray::new());
        let mut stat_data = (AliasMap::new(), AliasFinderArray::new());

        Self::add_base_expr_aliases(&mut expr_data);
        Self::add_base_stat_aliases(&mut stat_data);

        for &import in imports {
            Self::add_expr_aliases(&mut expr_data, import);
            Self::add_stat_aliases(&mut stat_data, import);
        }

        Self {
            expr_map: expr_data.0,
            stat_map: stat_data.0,
            expr_data: Arc::new(expr_data.1),
            stat_data: Arc::new(stat_data.1),
        }
    }
    fn add_aliases(data: &mut (AliasMap, AliasFinderArray), builder: AliasDataBuilder) {
        for (alias, func) in builder.map {
            if data.0.insert(alias, func).is_none() {
                data.1.add(alias);
            }
        }
    }
}
impl AliasData {
    fn add_base_expr_aliases(data: &mut (AliasMap, AliasFinderArray)) {
        Self::add_aliases(data, make_alias_data!(Addition))
    }
    fn add_base_stat_aliases(data: &mut (AliasMap, AliasFinderArray)) {
        Self::add_aliases(data, make_alias_data!(Stroke))
    }
    fn add_expr_aliases(data: &mut (AliasMap, AliasFinderArray), import: Import) {
        Self::add_aliases(
            data,
            match import {
                Import::List => make_alias_data!(),
                Import::Func => make_alias_data!(),
                Import::Graph => make_alias_data!(),
                Import::Frame => make_alias_data!(),
                Import::Trig => make_alias_data!(),
                Import::Rand => make_alias_data!(),
                Import::Stamp => make_alias_data!(),
            },
        )
    }
    fn add_stat_aliases(data: &mut (AliasMap, AliasFinderArray), import: Import) {
        Self::add_aliases(
            data,
            match import {
                Import::List => make_alias_data!(),
                Import::Func => make_alias_data!(),
                Import::Graph => make_alias_data!(),
                Import::Frame => make_alias_data!(),
                Import::Trig => make_alias_data!(),
                Import::Rand => make_alias_data!(),
                Import::Stamp => make_alias_data!(),
            },
        )
    }
}

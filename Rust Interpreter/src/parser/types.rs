use bitflags::bitflags;

bitflags! {
    #[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
    pub struct ReturnTypeSet: u32 {
        const Null =   0;
        const Void =   0b1;

        const Number = 0b10;
        const Bool =   0b100;
        const String = 0b1000;
        const Color =  0b10000;
        const List =   0b100000;
        const Any =    0b111110;
    }
}

impl ReturnTypeSet {
    pub fn intersect(parent_require: Self, child_give: Self) -> Option<Self> {
        let result = parent_require.intersection(child_give);
        (!result.is_empty()).then_some(result)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReturnType {
    Null,
    Void,
    Number,
    Bool,
    String,
    Color,
    List,
    Any,
}

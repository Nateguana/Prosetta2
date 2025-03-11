use bitflags::bitflags;

bitflags! {
    #[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
    pub struct Types: u32 {
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

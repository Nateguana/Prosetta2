use std::any::Any;

use super::{
    color_finder::ColorFinder,
    commands::{Parsable, ParseTreeObj},
    fail_reason::FailReason,
    parser_result::{ParserAction, ParserStep},
    slice::Slice,
    types::ReturnType, // slice::Slice,
                       // types::ReturnType,
};

// pub type Spot = Box<dyn Parsable>;

pub struct ParsableVec {
    inner: Vec<Box<dyn Parsable>>,
}

impl ParsableVec {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, element: Box<dyn Parsable>) -> usize {
        self.inner.push(element);
        self.inner.len()
    }

    pub fn drain(&mut self, to: usize) {
        self.inner.drain(to - 1..);
    }

    pub fn get_mut(&mut self, index: usize) -> &mut dyn Parsable {
        self.inner[index - 1].as_mut()
    }

    pub fn get(&self, index: usize) -> &dyn Parsable {
        self.inner[index - 1].as_ref()
    }

    pub fn into_root(self) -> Box<dyn Parsable> {
        self.inner[0]
    }
}

pub type StepContinueFunc<T: Parsable + 'static> =
    Box<dyn Fn(&mut T, Context, Slice<'_>) -> ParseResult<T>>;

pub type ChildContinueFunc<T: Parsable + 'static> =
    Box<dyn Fn(&mut T, Context, Slice<'_>, Option<(usize, ReturnType)>) -> ParseResult<T>>;

pub enum ChildType<T: Parsable + 'static> {
    Command(usize),
    Meta(Box<T>),
}

pub enum ParseResult<T: Parsable + 'static> {
    Match {
        pos: usize,
    },
    Fail {
        reason: FailReason,
    },
    Continue {
        pos: usize,
        description: String,
        step: StepContinueFunc<T>,
    },
    Child {
        child: ChildType<T>,
        step: ChildContinueFunc<T>,
        back: ChildContinueFunc<T>,
        slice: Slice<'static>,
    },
}

pub struct ContextBase {
    pub debug: bool,
    pub level: u8,
    pub color_finder: ColorFinder,
    pub vec: ParsableVec,
}

pub struct Context<'a> {
    base: &'a mut ContextBase,
    index: usize,
}

impl<'a> Context<'a> {
    pub fn get_level(&self) -> u8 {
        self.base.level
    }
    pub fn is_debug(&self) -> bool {
        self.base.debug
    }
    pub fn color_finder(&self) -> &ColorFinder {
        &self.color_finder
    }

    pub fn new_child(&mut self, child: impl Parsable) -> usize {
        self.base.vec.push(Box::new(child))
    }

    pub fn get<T>(&self, index: usize) -> Option<&T> {
        self.base.vec.get(index).as_any().downcast_ref::<T>()
    }
    pub fn get_mut<T>(&mut self, index: usize) -> Option<&mut T> {
        self.base.vec.get_mut(index).as_any().downcast_mut::<T>()
    }

    pub fn get_vec(&mut self) -> &mut ParsableVec {
        &mut self.vec
    }

    pub fn get_index() {}

    pub fn into_root<T>(self) -> Box<T> {
        Box::<dyn Any>::downcast(self.vec.into_root()).unwrap()
    }
}

// the context for the debug genenerator
// pub struct DebugContextBase {
//     base: RunContextBase,
// }

// pub struct RunContextBase {
//     color_finder: ColorFinder,
// }

// pub struct DebugContext<'a, 'b> {
//     base: &'a DebugContextBase,
//     inner: RunContext<'a, 'b>,
// }

// pub struct RunContext<'a, 'b> {
//     base: &'a RunContextBase,
//     level: u8,
// }

// impl DebugContextBase {
//     pub fn new() -> Self {
//         Self {
//             base: RunContextBase::new(),
//         }
//     }
// }

// impl RunContextBase {
//     pub fn new() -> Self {
//         Self {
//             color_finder: ColorFinder::new(),
//         }
//     }
// }

// impl<'a, 'b> DebugContext<'a, 'b> {
//     pub fn new(base: &'a DebugContextBase, parent: &'b dyn ParseTreeObj) -> Self {
//         Self {
//             base,
//             inner: RunContext::new(&base.base, parent),
//         }
//     }
//     pub fn new_from(&'a self, parent: &'b dyn ParseTreeObj) -> Self {
//         Self {
//             base: &self.base,
//             inner: RunContext::new_from(&self.inner, parent),
//         }
//     }
// }

// impl<'a, 'b> RunContext<'a, 'b> {
//     pub fn new(base: &'a RunContextBase) -> Self {
//         Self {
//             base,
//             level: 0,
//         }
//     }
//     pub fn new_from(&self) -> Self {
//         Self {
//             base: self.base;
//             level: self.level + 1,
//         }
//     }
// }

// #[async_trait::async_trait]
// impl<'a, 'b> Context for DebugContext<'a, 'b> {
//     async fn step_continue(&self, this: &dyn Parsable, pos: usize, description: String) {
//         self.base
//             .co
//             .yield_(ParserStep::new(
//                 ParserAction::Continue {
//                     child: this.get_name(),
//                     description,
//                 },
//                 pos,
//             ))
//             .await;
//     }
//     async fn step_child<T: Parsable + 'static>(
//         &self,
//         this: &dyn ParseTreeObj,
//         child: &T,
//         slice: Slice<'_>,
//     ) -> Option<(usize, ReturnType)> {
//         self.base
//             .co
//             .yield_(ParserStep::new(
//                 ParserAction::Child {
//                     parent: this.get_name(),
//                     child: child.get_name(),
//                 },
//                 slice.pos,
//             ))
//             .await;

//         let context = self.new_from(this);

//         let result = if self.inner.level < MAX_STACK_FRAME_LEVEL {
//             child.try_parse(context, slice).await
//         } else {
//             Err(FailReason::StackFrameLimit)
//         };

//         let parser_step = match result {
//             Ok((position, return_type)) => ParserStep::new(
//                 ParserAction::Matched {
//                     parent: this.get_name(),
//                     child: child.get_name(),
//                     return_type,
//                 },
//                 position,
//             ),
//             Err(reason) => ParserStep::new(
//                 ParserAction::Failed {
//                     parent: this.get_name(),
//                     child: child.get_name(),
//                     reason,
//                 },
//                 slice.pos,
//             ),
//         };

//         self.base.co.yield_(parser_step).await;

//         result.ok()
//     }

//     fn get_parent(&self) -> &'b dyn ParseTreeObj {
//         self.inner.parent
//     }

//     fn get_level(&self) -> u8 {
//         self.inner.get_level()
//     }
//     fn is_debug(&self) -> bool {
//         true
//     }

//     fn color_finder(&self) -> &ColorFinder {
//         &self.base.base.color_finder
//     }
//     // async fn step_paragraph(
//     //     &self,
//     //     this: &'static str,
//     //     child: &mut Box<dyn Command>,
//     //     slice: Slice<'_>,
//     // ) -> Option<(usize, ReturnType)> {
//     //     // let command = child.as_mut();
//     //     // let parse_result = command.get_next_call(self, slice).await;
//     //     // match parse_result {
//     //     //     Ok(ret @ (pos, _)) => {
//     //     //         self.step_match(this, command, pos).await;
//     //     //         Some(ret)
//     //     //     }
//     //     //     Err(_fail_reason) => None,
//     //     // }
//     //     None
//     // }
//     // async fn step_child<T: Command + 'static>(
//     //     &self,
//     //     this: &dyn Command,
//     //     spot: &mut Box<dyn Command>,
//     //     slice: Slice<'_>,
//     // ) -> Option<(usize, ReturnType)> {
//     //     *spot = Box::new(T::new());
//     //     self.step_paragraph(this.name(), spot, slice).await
//     // }
// }
// // #[async_trait::async_trait]
// impl<'a, 'b> Context for RunContext<'a, 'b> {
//     // async fn step_continue(&self, _this: &dyn Parsable, _pos: usize, _description: String) {}
//     // async fn step_child<T: Parsable + 'static>(
//     //     &self,
//     //     this: &dyn ParseTreeObj,
//     //     child: &T,
//     //     slice: Slice<'_>,
//     // ) -> Option<(usize, ReturnType)> {
//     //     let context = self.new_from(this);

//     //     if self.level < MAX_STACK_FRAME_LEVEL {
//     //         let ret = child.try_parse(context, slice).await;
//     //         ret.ok()
//     //     } else {
//     //         None
//     //     }
//     // }

//     fn get_parent(&self) -> &'b dyn ParseTreeObj {
//         self.parent
//     }

//     fn get_level(&self) -> u8 {
//         self.level
//     }

//     fn is_debug(&self) -> bool {
//         false
//     }

//     fn color_finder(&self) -> &ColorFinder {
//         &self.base.color_finder
//     }
// }

// // async fn step_child_impl<T: Command + 'static>(
// //     co: impl Context,
// //     child: &T,
// //     slice: Slice<'_>,
// // ) -> Result<(usize, ReturnType), FailReason> {
// //     if co.get_level() <= MAX_STACK_FRAME_LEVEL {
// //         child.try_parse(co, slice).await;
// //     } else {
// //         None
// //     }
// // }

// // #[macro_export]
// // macro_rules! Step_Continue {
// //     ($co:expr,$self:expr,$pos:expr,$format:expr) => {
// //         $co.step_continue(
// //             $self,
// //             $pos,
// //             format!($format, $self.get_name()),
// //         )
// //         .await;
// //     };
// //     ($co:expr,$self:expr,$pos:expr,$format:expr,$($args:expr),*) => {
// //         $co.step_continue(
// //             $self,
// //             $pos,
// //             format!($format, $self.get_name(), $($args:expr), *),
// //         )
// //         .await;
// //     };
// // }

// // pub(crate) use Step_Continue;

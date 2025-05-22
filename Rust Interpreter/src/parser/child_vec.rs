use std::ops::Deref;

use super::{commands::ParseTreeObj, rwlock::RwLockReadGuard};

pub struct ChildVec<'a, T: ParseTreeObj + ?Sized> {
    inner: Result<RwLockReadGuard<'a, Vec<Box<T>>>, Vec<Box<T>>>,
}

impl<'a, T: ParseTreeObj + ?Sized> ChildVec<'a, T> {
    pub fn empty() -> Self {
        Self {
            inner: Err(Vec::new()),
        }
    }

    pub fn new() -> Self {
        Self {
            inner: Err(Vec::new()),
        }
    }
}

impl<'a, T: ParseTreeObj + ?Sized> Deref for ChildVec<'a, T> {
    type Target = Vec<Box<T>>;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().map_or_else(|e| e, |e| e)
    }
}

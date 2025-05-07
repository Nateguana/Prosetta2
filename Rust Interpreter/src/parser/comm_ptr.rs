use std::{any::Any, marker::PhantomData, ops::DerefMut};

use parking_lot::{Mutex, MutexGuard};

use super::commands::Command;

pub struct CommPtr<'a, T> {
    tree: &'a Mutex<Box<dyn Command>>,
    index: usize,
    _marker: PhantomData<T>,
}

impl<'a, T> core::ops::Receiver for CommPtr<'a, T> {
    type Target = T;
}

fn map_ptr<'a, T: 'static>(value: &'a mut Box<dyn Command + 'static>) -> &'a mut T {
    let ret = (value.as_mut() as &mut dyn Any).downcast_mut::<T>();
    ret.unwrap()
}

impl<'a, T: 'static> CommPtr<'a, T> {
    pub fn value(&self) -> impl DerefMut<Target = T> + 'a {
        let ret = MutexGuard::map(self.tree.lock(), map_ptr);
        ret
    }
}

use std::{any::Any, marker::PhantomData, ops::DerefMut, sync::Arc};

use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};

use super::commands::Command;

type Arena = Vec<Box<dyn Command + 'static>>;
pub struct CommPtr<T> {
    tree: Arc<Mutex<Arena>>,
    index: usize,
    _marker: PhantomData<T>,
}

impl<T> core::ops::Receiver for CommPtr<T> {
    type Target = T;
}

impl<T: 'static> CommPtr<T> {
    pub fn value<'a: 'static, 'b>(&'b self) -> impl DerefMut<Target = T> + 'static {
        // fn map_ptr<'b, T: 'static>(value: &'b mut Arena, index: usize) -> &'b mut T {
        //     let ret = (value[self.index].as_mut() as &mut dyn Any).downcast_mut::<T>().unwrap()
        //     ret.unwrap()
        // }
        // MutexGuard::map
        // SAFETY: this code is take directly from MutexGuard::map. I needed a self reference that I couldn't pass
        // let s = self.tree.lock();
        // unsafe {
        //     let raw = MutexGuard::mutex(&s);
        //     let data = f(unsafe { &mut *s.mutex.data.get() });
        //     mem::forget(s);
        //     MappedMutexGuard {
        //         raw,
        //         data,
        //         marker: PhantomData,
        //     }
        // }
        // let
        let index = self.index;
        let map_fn: &dyn Fn(&'b mut Arena) -> &'b mut T = &|value| Self::map_ptr2(value);
        let other_fn = Self::map_ptr;

        let ret = MutexGuard::map(self.tree.lock(), Self::map_ptr2);
        ret
    }

    fn map_ptr2<'a>(value: &'a mut Arena) -> &'a mut T {
        let ret = (value[0].as_mut() as &mut dyn Any).downcast_mut::<T>();
        ret.unwrap()
    }

    fn map_ptr<'a>(&self, value: &'a mut Arena) -> &'a mut T {
        let ret = (value[self.index].as_mut() as &mut dyn Any).downcast_mut::<T>();
        ret.unwrap()
    }
}

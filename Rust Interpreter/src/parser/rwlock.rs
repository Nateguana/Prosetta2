use std::{any::Any, marker::PhantomData, ops::Deref, sync::Arc};

use tokio::sync;

#[allow(unused)]
pub(crate) use sync::{RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Default)]
pub struct RwLock<T> {
    inner: sync::RwLock<T>,
}

impl<T> RwLock<T> {
    pub fn new(t: T) -> Self {
        Self {
            inner: sync::RwLock::new(t),
        }
    }

    pub fn read(&self) -> sync::RwLockReadGuard<'_, T> {
        self.inner.try_read().unwrap()
    }

    pub fn write(&self) -> sync::RwLockWriteGuard<'_, T> {
        self.inner.try_write().unwrap()
    }

    pub fn read_map<F, U: ?Sized>(&self, f: F) -> sync::RwLockReadGuard<'_, U>
    where
        F: FnOnce(&T) -> &U,
    {
        RwLockReadGuard::map(self.read(), f)
    }

    // pub fn read_map<'a, 'b: 'a, U: 'a, F>(&'b self, f: F) -> RwLockMappedReadGuard<'a, 'b, U>
    // where
    //     F: FnOnce(&'a sync::RwLockReadGuard<'b, T>) -> U,
    //     T: 'a,
    // {
    //     RwLockMappedReadGuard::new(self.read(), f)
    // }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

// pub struct RwLockMappedReadGuard<'a, 'b: 'a, T: 'a> {
//     inner: T,
//     guard: Box<(dyn Deref<> + 'b)>,
//     _phantom: PhantomData<&'a T>,
// }

// impl<'a, 'b: 'a, T: 'a> RwLockMappedReadGuard<'a, 'b, T> {
//     pub fn empty<F>(f: F) -> Self
//     where
//         F: FnOnce() -> T,
//     {
//         Self {
//             inner: (f)(),
//             guard: Box::new(Vec::<()>::new()),
//             _phantom: PhantomData,
//         }
//     }

//     fn new<F, U>(guard: sync::RwLockReadGuard<'b, U>, f: F) -> Self
//     where
//         F: FnOnce(&'a sync::RwLockReadGuard<'b, U>) -> T,
//     {
//         let inner = (f)(&guard);
//         Self {
//             inner,
//             guard: Box::new(guard),
//             _phantom: PhantomData,
//         }
//     }

//     fn get(self) -> T {
//         self.inner
//     }
// }s

// impl<'a, T, U, F> std::ops::Deref for RwLockMappedReadGuard<'a, T, U, F> {
//     type Target = U;

//     fn deref(&self) -> &Self::Target {
//         &self.get();
//     }
// }

#[derive(Debug, Default)]
pub struct ArcRwLock<T> {
    inner: Arc<sync::RwLock<T>>,
}

impl<T> ArcRwLock<T> {
    pub fn new(t: T) -> Self {
        Self {
            inner: Arc::new(sync::RwLock::new(t)),
        }
    }

    pub fn read(&self) -> sync::RwLockReadGuard<'_, T> {
        self.inner.try_read().unwrap()
    }

    pub fn write(&self) -> sync::RwLockWriteGuard<'_, T> {
        self.inner.try_write().unwrap()
    }

    pub fn into_inner(self) -> T {
        Arc::into_inner(self.inner).unwrap().into_inner()
    }
}

impl<T> Clone for ArcRwLock<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

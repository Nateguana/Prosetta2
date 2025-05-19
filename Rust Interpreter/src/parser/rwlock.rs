use std::sync::Arc;

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

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

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

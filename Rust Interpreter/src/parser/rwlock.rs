use std::sync::Arc;

use smol::lock;

#[allow(unused)]
pub(crate) use lock::{RwLockReadGuard, RwLockReadGuardArc, RwLockWriteGuard, RwLockWriteGuardArc};

#[derive(Debug, Default)]
pub struct RwLock<T> {
    inner: lock::RwLock<T>,
}

impl<T> RwLock<T> {
    pub const fn new(t: T) -> Self {
        Self {
            inner: lock::RwLock::new(t),
        }
    }

    pub fn read(&self) -> lock::RwLockReadGuard<'_, T> {
        self.inner.try_read().unwrap()
    }

    pub fn write(&self) -> lock::RwLockWriteGuard<'_, T> {
        self.inner.try_write().unwrap()
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

#[derive(Debug, Default)]
pub struct ArcRwLock<T> {
    inner: Arc<lock::RwLock<T>>,
}

impl<T> ArcRwLock<T> {
    pub fn new(t: T) -> Self {
        Self {
            inner: Arc::new(lock::RwLock::new(t)),
        }
    }

    pub fn read(&self) -> lock::RwLockReadGuardArc<T> {
        self.inner.try_read_arc().unwrap()
    }

    pub fn write(&self) -> lock::RwLockWriteGuardArc<T> {
        self.inner.try_write_arc().unwrap()
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

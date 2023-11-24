use core::{cell::RefCell, ops::{Deref, DerefMut}};

pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

/// SAFETY: The user must garantee that UPSafeCell is only used in
/// single-threaded context.
unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
}

impl<T> Deref for UPSafeCell<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for UPSafeCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

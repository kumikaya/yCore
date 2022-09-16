use core::cell::RefCell;
use core::ops::Deref;


pub struct STCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T: Sync> Sync for STCell<T> { }

impl<T> STCell<T> {
    pub fn new(data: T) -> Self {
        Self { inner: RefCell::new(data) }
    }
}

impl<T> Deref for STCell<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
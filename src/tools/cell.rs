use core::cell::{RefCell, Cell};
use core::ops::Deref;


pub struct STRefCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T: Sync> Sync for STRefCell<T> { }

impl<T> STRefCell<T> {
    pub fn new(data: T) -> Self {
        Self { inner: RefCell::new(data) }
    }
}

impl<T> Deref for STRefCell<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct STCell<T> {
    inner: Cell<T>,
}

unsafe impl<T: Sync> Sync for STCell<T> { }

impl<T> STCell<T> {
    pub fn new(data: T) -> Self {
        Self { inner: Cell::new(data) }
    }
}

impl<T> Deref for STCell<T> {
    type Target = Cell<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
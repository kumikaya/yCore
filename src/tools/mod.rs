use core::slice;

use alloc::vec::Vec;
pub mod ansi;
pub mod logging;
pub mod ring_buffer;
use anyhow::Result;

// pub const fn align_size<T>(align: usize) -> usize {
//     let size = size_of::<T>();
//     (size - 1 + align) - ((size - 1) % align)
// }

pub const fn align_ceil(val: usize, align: usize) -> usize {
    (val - 1 + align) - ((val - 1) % align)
}

pub unsafe fn from_cstr(ptr: *const u8) -> &'static str {
    let mut end = ptr;
    while end.read() != b'\0' {
        end = end.add(1);
    }
    core::str::from_utf8(slice::from_raw_parts(ptr, end as usize - ptr as usize)).unwrap()
}

#[derive(Clone, Default)]
pub struct Table<T> {
    inner: Vec<Option<T>>,
}

impl<T> FromIterator<Option<T>> for Table<T> {
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with(mut self, val: T) -> Self {
        self.push(val);
        self
    }

    pub fn alloc_id(&mut self) -> usize {
        let idx_may = self
            .inner
            .iter()
            .enumerate()
            .find(|(_, file)| file.is_none())
            .map(|(idx, _)| idx);
        if let Some(idx) = idx_may {
            idx
        } else {
            self.inner.push(None);
            self.inner.len() - 1
        }
    }

    pub fn push_opt(&mut self, val: Option<T>) -> usize {
        let idx = self.alloc_id();
        *self.get_entry(idx) = val;
        idx
    }

    pub fn push(&mut self, val: T) -> usize {
        let idx = self.alloc_id();
        *self.get_entry(idx) = Some(val);
        idx
    }
    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if let Some(val) = self.inner.get_mut(idx) {
            val.take()
        } else {
            None
        }
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        if let Some(val) = self.inner.get(idx) {
            val.as_ref()
        } else {
            None
        }
    }
    pub fn get_entry(&mut self, idx: usize) -> &mut Option<T> {
        self.inner.get_mut(idx).unwrap()
    }
    pub fn swap(&mut self, idx0: usize, idx1: usize) -> Result<()> {
        if self.get(idx0).is_some() && self.get(idx1).is_some() {
            self.inner.swap(idx0, idx1);
            Ok(())
        } else {
            Err(anyhow!("exchange failed, some index did not exist"))
        }
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn iter_elem(&self) -> impl Iterator<Item = &T> {
        self.inner.iter().filter_map(|x| x.as_ref())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<T>> {
        self.inner.iter()
    }
}

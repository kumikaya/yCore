use core::{mem::size_of, slice};
pub mod ansi;
pub mod logging;

pub const fn aligned_size<T>(meta: usize) -> usize {
    let raw_size = size_of::<T>();
    let r = raw_size % meta;
    if r == 0 {
        raw_size
    } else {
        raw_size + meta - r
    }
}

pub unsafe fn from_cstr(ptr: *const u8) -> &'static str {
    let mut end = ptr;
    while end.read() != '\0' as u8 {
        end = end.add(1);
    }
    core::str::from_utf8(slice::from_raw_parts(ptr, end as usize - ptr as usize)).unwrap()
}

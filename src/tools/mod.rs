use core::{mem::size_of, slice};
pub mod ansi;
pub mod logging;

pub const fn align_ceil<T>(meta: usize) -> usize {
    let align_size = size_of::<T>();
    (meta - 1 + align_size) - ((meta - 1) % align_size)
}

pub unsafe fn from_cstr(ptr: *const u8) -> &'static str {
    let mut end = ptr;
    while end.read() != '\0' as u8 {
        end = end.add(1);
    }
    core::str::from_utf8(slice::from_raw_parts(ptr, end as usize - ptr as usize)).unwrap()
}

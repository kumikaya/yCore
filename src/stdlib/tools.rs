use core::mem::size_of;


pub const fn aligned_size<T>(meta: usize) -> usize {
    let raw_size = size_of::<T>();
    let r = raw_size % meta;
    if r == 0 {
        raw_size
    } else {
        raw_size + meta - r
    }
}
use core::slice;

use crate::print;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let s: &str = unsafe {
                core::str::from_utf8(slice::from_raw_parts(buf, len)).unwrap()
            };
            print!("{}", s);
            len as isize
        },
        _ => todo!(),
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    todo!()
}
use core::slice;

use crate::{print, mem::{memory_set::KERNEL_SPACE, address::VirtAddr}, println, task::user_addr_translate};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: VirtAddr, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let phy_addr = user_addr_translate(buf).unwrap();
            let s: &str = unsafe {
                core::str::from_utf8(slice::from_raw_parts(phy_addr.0 as *const u8, len)).unwrap()
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
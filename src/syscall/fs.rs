use crate::{
    config::PAGE_SIZE,
    mem::{address::VirtAddr, page_table::translated_byte_buffer},
    print,
    task::user_space,
};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: VirtAddr, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            assert!(len <= PAGE_SIZE, "unimplemented");
            let buffers = translated_byte_buffer(user_space(), buf, len);
            for buffer in buffers {
                let s: &str = unsafe {
                    core::str::from_utf8_unchecked(&*buffer)
                };
                print!("{}", s);
            }
            len as isize
        }
        _ => todo!(),
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    todo!()
}

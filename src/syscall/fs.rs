
use crate::{
    mm::{
        address::VirtAddr,
        page_table::{translated_byte_buffer, translated_string, BufferHandle},
    },
    task::processor::Hart, fs::inode::{open_file, OpenFlags},
};

pub(super) trait SysFs {
    fn sys_write(&self, fd: usize, buf: usize, len: usize) -> isize;
    fn sys_read(&self, fd: usize, buf: usize, len: usize) -> isize;
    fn sys_open(&self, ptr: VirtAddr, len: usize, flags: u32) -> isize;
    fn sys_close(&self, fd: usize) -> isize;
}

impl<T: Hart> SysFs for T {
    fn sys_write(&self, fd: usize, buf: usize, len: usize) -> isize {
        let task = self.current_task();
        let fd_table = task.fd_table.borrow();
        if let Some(file) = fd_table.get(fd) {
            if file.writable() {
                let buffer = unsafe {
                    BufferHandle::new(translated_byte_buffer(task.space(), buf.into(), len))
                };
                return file.write(buffer) as isize;
            }
        }
        -1
    }

    fn sys_read(&self, fd: usize, buf: usize, len: usize) -> isize {
        let task = self.current_task();
        let fd_table = task.fd_table.borrow();
        if let Some(file) = fd_table.get(fd) {
            if file.readable() {
                let buffer = unsafe {
                    BufferHandle::new(translated_byte_buffer(task.space(), buf.into(), len))
                };
                return file.read(buffer) as isize;
            }
        }
        -1
    }

    fn sys_open(&self, ptr: VirtAddr, len: usize, flags: u32) -> isize {
        let task = self.current_task();
        let path = unsafe { translated_string(task.space(), ptr, len) };
        let flags = OpenFlags::from_bits_truncate(flags as u8);
        if let Some(inode) = open_file(&path, flags) {
            task.fd_table.borrow_mut().push_fd(inode) as isize
        } else {
            -1
        }
    }

    fn sys_close(&self, fd: usize) -> isize {
        let task = self.current_task();
        let mut fd_table = task.fd_table.borrow_mut();
        if let Some(_) = fd_table.close(fd) {
            0
        } else {
            -1
        }
    }
}

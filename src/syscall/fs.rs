use crate::{
    mem::{
        address::VirtAddr,
        page_table::{translated_byte_buffer, translated_string, UserBuffer},
    },
    task::processor::Hart,
};

pub(super) trait FS {
    fn sys_write(&self, fd: usize, buf: usize, len: usize) -> isize;
    fn sys_read(&self, fd: usize, buf: usize, len: usize) -> isize;
    fn sys_open(&self, ptr: VirtAddr, len: usize, flags: u32) -> isize;
    fn sys_close(&self, fd: usize) -> isize;
}

impl<T: Hart> FS for T {
    fn sys_write(&self, fd: usize, buf: usize, len: usize) -> isize {
        let task = self.current_task();
        let fd_table = &task.context.borrow().fd_table;
        if let Some(Some(file)) = fd_table.get(fd) {
            if file.writable() {
                let buffer = unsafe {
                    UserBuffer::new(translated_byte_buffer(task.space(), buf.into(), len))
                };
                return file.write(&buffer) as isize;
            }
        }
        -1
    }

    fn sys_read(&self, fd: usize, buf: usize, len: usize) -> isize {
        let task = self.current_task();
        let fd_table = &task.context.borrow().fd_table;
        if let Some(Some(file)) = fd_table.get(fd) {
            if file.readable() {
                let mut buffer = unsafe {
                    UserBuffer::new(translated_byte_buffer(task.space(), buf.into(), len))
                };
                return file.read(&mut buffer) as isize;
            }
        }
        -1
    }

    fn sys_open(&self, ptr: VirtAddr, len: usize, flags: u32) -> isize {
        let task = self.current_task();
        let path = unsafe { translated_string(task.space(), ptr, len) };
        todo!()
    }

    fn sys_close(&self, fd: usize) -> isize {
        let task = self.current_task();
        let fd_table = &mut task.context.borrow_mut().fd_table;
        if let Some(Some(_)) = fd_table.get_mut(fd).take() {
            0
        } else {
            -1
        }
    }
}

use crate::{
    fs::{
        inode::{open_file, OpenFlags},
        pipe::make_pipe,
    },
    mm::{
        address::VirtAddr,
        page_table::{translated_byte_buffer, translated_refmut, translated_string, BufferHandle},
    },
    syscall_unwarp,
    task::processor::Schedule,
};

use super::{EXEC_FAIL, EXEC_SUCCEE};

pub(super) trait SysFs {
    fn sys_write(&self, fd: usize, buf: usize, len: usize) -> isize;
    fn sys_read(&self, fd: usize, buf: usize, len: usize) -> isize;
    fn sys_open(&self, ptr: VirtAddr, len: usize, flags: u32) -> isize;
    fn sys_close(&self, fd: usize) -> isize;
    fn sys_pipe(&self, pipe: *mut usize) -> isize;
    fn sys_dup(&self, fd: usize) -> isize;
}

impl<T: Schedule> SysFs for T {
    fn sys_write(&self, fd: usize, buf: usize, len: usize) -> isize {
        let task = self.current_task();
        let local = task.local.borrow();
        let fd_table = &local.fd_table;
        if let Some(file) = fd_table.get(fd) {
            if file.writable() {
                let buffer = unsafe {
                    BufferHandle::new(syscall_unwarp!(translated_byte_buffer(
                        task.space(),
                        buf.into(),
                        len
                    )))
                };
                return file.write(buffer) as isize;
            }
        }
        EXEC_FAIL
    }

    fn sys_read(&self, fd: usize, buf: usize, len: usize) -> isize {
        let task = self.current_task();
        let local = task.local.borrow();
        let fd_table = &local.fd_table;
        if let Some(file) = fd_table.get(fd) {
            if file.readable() {
                let buffer = unsafe {
                    BufferHandle::new(syscall_unwarp!(translated_byte_buffer(
                        task.space(),
                        buf.into(),
                        len
                    )))
                };
                return file.read(buffer) as isize;
            }
        }
        EXEC_FAIL
    }

    fn sys_open(&self, ptr: VirtAddr, len: usize, flags: u32) -> isize {
        let task = self.current_task();
        let path = unsafe { syscall_unwarp!(translated_string(task.space(), ptr, len)) };
        let flags = OpenFlags::from_bits_truncate(flags as u8);
        if let Some(inode) = open_file(&path, flags) {
            task.local.borrow_mut().fd_table.push_fd(inode) as isize
        } else {
            EXEC_FAIL
        }
    }

    fn sys_close(&self, fd: usize) -> isize {
        let task = self.current_task();
        let mut local = task.local.borrow_mut();
        if local.fd_table.close(fd).is_some() {
            EXEC_SUCCEE
        } else {
            EXEC_FAIL
        }
    }

    fn sys_pipe(&self, pipe: *mut usize) -> isize {
        let task = self.current_task();
        let space = task.space();
        let (pipe_read, pipe_write) = make_pipe();
        let mut local = task.local.borrow_mut();
        let fd_table = &mut local.fd_table;
        let read_fd = fd_table.push_fd(pipe_read);
        let write_fd = fd_table.push_fd(pipe_write);
        unsafe {
            let read_fd_ptr = syscall_unwarp!(translated_refmut(space, pipe));
            *read_fd_ptr = read_fd;
            let write_fd_ptr = syscall_unwarp!(translated_refmut(space, pipe.add(1)));
            *write_fd_ptr = write_fd;
        }
        EXEC_SUCCEE
    }
    fn sys_dup(&self, fd: usize) -> isize {
        let task = self.current_task();
        let mut local = task.local.borrow_mut();
        let fd_table = &mut local.fd_table;
        if let Some(file) = fd_table.get(fd) {
            let cp_file = file.clone();
            fd_table.push_fd(cp_file) as isize
        } else {
            EXEC_FAIL
        }
    }
}

mod fs;
mod process;
mod sys;
mod sync;
use crate::task::processor::Hart;

use self::{fs::*, process::*, sys::*, sync::*};

const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_SLEEP: usize = 101;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_TIME: usize = 169;
const SYSCALL_GET_PID: usize = 172;

const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_MMAP: usize = 222;

// const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;

pub trait Syscall {
    fn syscall(&self, syscall_id: usize, args: (usize, usize, usize)) -> isize;
}

impl<T: Hart> Syscall for T {
    #[inline]
    fn syscall(&self, syscall_id: usize, args: (usize, usize, usize)) -> isize {
        match syscall_id {
            SYSCALL_READ    => self.sys_read(args.0, args.1, args.2),
            SYSCALL_WRITE   => self.sys_write(args.0, args.1, args.2),
            SYSCALL_EXIT    => self.sys_exit(args.0 as i32),
            SYSCALL_YIELD   => self.sys_yield(),
            SYSCALL_TIME    => sys_get_time(),
            SYSCALL_GET_PID => self.sys_get_pid(),
            SYSCALL_SLEEP   => self.sys_sleep(args.0),
            SYSCALL_MUNMAP  => self.sys_munmap(args.0.into(), args.1),
            SYSCALL_MMAP    => self.sys_mmap(args.0.into(), args.1, args.2),
            // SYSCALL_FORK    => sys_fork(),
            SYSCALL_EXEC    => self.sys_exec(args.0.into(), args.1),
            SYSCALL_WAITPID => self.sys_waitpid(args.0 as isize, args.1 as *mut i32),
            _ => todo!(),
        }
    }
}
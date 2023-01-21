mod fs;
mod mm;
mod process;
mod sync;
use log::warn;

use self::{fs::*, mm::*, process::*, sync::*};
use crate::task::processor::Schedule;

const SYSCALL_DUP: usize = 24;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_SLEEP: usize = 101;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_SIGACTION: usize = 134;
const SYSCALL_SIGPROCMASK: usize = 135;
const SYSCALL_SIGRETURN: usize = 139;
const SYSCALL_TIME: usize = 169;
const SYSCALL_GET_PID: usize = 172;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXECVE: usize = 221;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_WAITPID: usize = 260;

const EXEC_SUCCEE: isize = 0;
const EXEC_FAIL: isize = -1;

pub trait Syscall {
    fn syscall(&self, syscall_id: usize, args: [usize; 6]) -> isize;
}

impl<T: Schedule> Syscall for T {
    #[inline(always)]
    fn syscall(&self, syscall_id: usize, args: [usize; 6]) -> isize {
        match syscall_id {
            SYSCALL_DUP => self.sys_dup(args[0]),
            SYSCALL_OPEN => self.sys_open(args[0].into(), args[1], args[2] as u32),
            SYSCALL_CLOSE => self.sys_close(args[0]),
            SYSCALL_PIPE => self.sys_pipe(args[0] as *mut usize),
            SYSCALL_READ => self.sys_read(args[0], args[1], args[2]),
            SYSCALL_WRITE => self.sys_write(args[0], args[1], args[2]),
            SYSCALL_EXIT => self.sys_exit(args[0] as i32),
            SYSCALL_YIELD => self.sys_yield(),
            SYSCALL_TIME => sys_get_time(),
            SYSCALL_GET_PID => self.sys_get_pid(),
            SYSCALL_SLEEP => self.sys_sleep(args[0]),
            SYSCALL_MUNMAP => self.sys_munmap(args[0].into(), args[1]),
            SYSCALL_MMAP => self.sys_mmap(args[0].into(), args[1], args[2], args[3]),
            SYSCALL_FORK => self.sys_fork(),
            SYSCALL_EXECVE => self.sys_exec(args[0].into(), args[1], args[2] as u32),
            SYSCALL_WAITPID => self.sys_waitpid(args[0] as isize, args[1] as *mut i32),
            _ => {
                warn!("Unsupported syscall id: {}", syscall_id);
                -1
            }
        }
    }
}

#[macro_export]
macro_rules! syscall_unwarp {
    ($exp: expr) => {
        match $exp {
            Ok(value) => value,
            Err(err) => {
                log::warn!("{}", err);
                return EXEC_FAIL;
            }
        }
    };
}

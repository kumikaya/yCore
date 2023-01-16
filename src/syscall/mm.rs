use log::warn;

use crate::{
    mm::{address::VirtAddr, memory_set::MapPerm, page_table::PTEFlags}, task::processor::Schedule, syscall::EXEC_FAIL, syscall_unwarp,
};

use super::EXEC_SUCCEE;


pub(super) trait SysMm {
    fn sys_munmap(&self, va: VirtAddr, len: usize) -> isize;
    fn sys_mmap(&self, va: VirtAddr, len: usize, perm: usize, fd: usize) -> isize;
}


impl<T: Schedule> SysMm for T {
    fn sys_munmap(&self, va: VirtAddr, len: usize) -> isize {
        let range = va.floor()..va.offset(len as isize).ceil();
        let user_space = self.current_task().space();
        for vpn in range {
            syscall_unwarp!(user_space.free(vpn));
        }
        EXEC_SUCCEE
    }

    fn sys_mmap(&self, va: VirtAddr, len: usize, perm: usize, _fd: usize) -> isize {
        let perm = MapPerm::from_bits_truncate(perm as u8);
        assert_ne!(perm & MapPerm::RWX, MapPerm::empty());
        let flags = PTEFlags::from_bits_truncate(perm.bits());
        let range = va.floor()..va.offset(len as isize).ceil();
        let user_space = self.current_task().space();
        for vpn in range {
            syscall_unwarp!(user_space.malloc(vpn, flags));
        }
        EXEC_SUCCEE
    }
}

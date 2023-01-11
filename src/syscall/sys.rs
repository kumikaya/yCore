use crate::{
    mem::{address::VirtAddr, memory_set::MapPerm, page_table::PTEFlags},
    task::processor::Hart,
};

pub(super) trait Sys {
    fn sys_munmap(&self, va: VirtAddr, len: usize) -> isize;
    fn sys_mmap(&self, va: VirtAddr, len: usize, perm: usize) -> isize;
}

impl<T: Hart> Sys for T {
    fn sys_munmap(&self, va: VirtAddr, len: usize) -> isize {
        let range = va.floor()..va.offset(len as isize).ceil();
        let user_space = self.current_task().space();
        for vpn in range {
            user_space.free(vpn).unwrap();
        }
        0
    }

    fn sys_mmap(&self, va: VirtAddr, len: usize, perm: usize) -> isize {
        let perm = MapPerm::from_bits_truncate(perm as u8);
        assert_ne!(perm & MapPerm::RWX, MapPerm::empty());
        let flags = PTEFlags::from_bits_truncate(perm.bits());
        let range = va.floor()..va.offset(len as isize).ceil();
        let user_space = self.current_task().space();
        for vpn in range {
            user_space.malloc(vpn, flags).unwrap();
        }
        0
    }
}

use core::arch::asm;

use alloc::vec::Vec;
use spin::{Lazy, Mutex};

use crate::{
    config::{kernel_stack_position, KERNEL_STACK_SIZE},
    mm::{
        address::VirtAddr,
        memory_set::{add_kernel_stack, remove_kernel_stack},
    },
};

#[derive(Debug)]
pub struct Pid {
    pub id: isize,
}

struct UidAllocator {
    current: isize,
    recycled: Vec<isize>,
}

impl UidAllocator {
    pub fn new() -> Self {
        UidAllocator {
            current: 1,
            recycled: Vec::new(),
        }
    }
    pub fn alloc(&mut self) -> isize {
        if let Some(pid) = self.recycled.pop() {
            pid
        } else {
            self.current += 1;
            self.current - 1
        }
    }
    pub fn dealloc(&mut self, pid: isize) {
        assert!(pid < self.current);
        assert!(
            !self.recycled.iter().any(|&rpid| rpid == pid),
            "pid {pid} has been deallocated!"
        );
        self.recycled.push(pid);
    }
}

static PID_ALLOCATOR: Lazy<Mutex<UidAllocator>> = Lazy::new(|| Mutex::new(UidAllocator::new()));

static KSTACK_UID_ALLOCATOR: Lazy<Mutex<UidAllocator>> =
    Lazy::new(|| Mutex::new(UidAllocator::new()));

pub fn pid_alloc() -> Pid {
    Pid {
        id: PID_ALLOCATOR.lock().alloc(),
    }
}

pub struct KernelStack {
    id: isize,
}

pub fn kstack_alloc() -> KernelStack {
    let kstack_id = KSTACK_UID_ALLOCATOR.lock().alloc();
    let (kstack_top, kstack_bottom) = kernel_stack_position(kstack_id);
    unsafe { add_kernel_stack(kstack_top.into(), kstack_bottom.into()) };
    KernelStack { id: kstack_id }
}

#[inline(always)]
fn get_sp() -> usize {
    let ret: usize;
    unsafe {
        asm! {r"
            mv {ret}, sp
            ",
            ret = out(reg) ret
        }
    }
    ret
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let (_, ksp) = kernel_stack_position(self.id);
        assert!(!(ksp..(ksp + KERNEL_STACK_SIZE)).contains(&get_sp()));
        unsafe { remove_kernel_stack(VirtAddr::from(ksp).into()) };
        KSTACK_UID_ALLOCATOR.lock().dealloc(self.id);
    }
}

impl KernelStack {
    // pub fn push<T>(&self, value: T) -> *mut T
    // where
    //     T: Sized,
    // {
    //     let kstack_bottom = self.bottom();
    //     let ptr_mut = (kstack_bottom - core::mem::size_of::<T>()) as *mut T;
    //     unsafe {
    //         *ptr_mut = value;
    //     }
    //     ptr_mut
    // }
    pub fn bottom(&self) -> usize {
        let (_, kstack_bottom) = kernel_stack_position(self.id);
        kstack_bottom
    }
}

impl Drop for Pid {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().dealloc(self.id);
    }
}

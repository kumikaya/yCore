use crate::{
    config::KERNEL_INIT_STACK_SIZE,
    mm::{
        address::{PhysAddr, VirtAddr},
        memory_set::MemorySet,
    },
    trap::{context::TrapContext, init_app_trap_return},
    KERNEL_STACK,
};

use super::processor::switch_trampoline;

pub struct Context {
    pub task_cx: TaskContext,
    pub trap_cx: PhysAddr,
}

impl Context {
    pub fn build(memory_set: &MemorySet, trap_cx: TrapContext, trap_va: VirtAddr) -> Self {
        let cx_pa = memory_set.va_translate(trap_va).unwrap();
        let satp = memory_set.token();
        unsafe {
            *cx_pa.as_type() = trap_cx;
        }
        Self {
            task_cx: TaskContext::goto_trap_return(trap_cx.ksp, satp, trap_va.into()),
            trap_cx: cx_pa,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,      // 0
    ksp: usize,     // 1
    s: [usize; 12], // 2
}

// impl const Default for TaskContext {
//     fn default() -> Self {
//         Self {
//             ra: 0,
//             ksp: 0,
//             s: [0; 12],
//         }
//     }
// }

impl TaskContext {
    pub fn goto_trap_return(ksp: usize, satp: usize, trap_cx: usize) -> Self {
        let mut result = Self {
            ra: init_app_trap_return as usize,
            ksp,
            s: [0; 12],
        };
        result.s[0] = satp;
        result.s[1] = trap_cx;
        result
    }

    pub fn switch_trampoline(hartid: usize) -> Self {
        let ksp =
            unsafe { &KERNEL_STACK as *const u8 as usize } + (hartid + 1) * KERNEL_INIT_STACK_SIZE;
        Self {
            ra: switch_trampoline as usize,
            ksp,
            s: [0; 12],
        }
    }
}

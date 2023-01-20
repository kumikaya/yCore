

use crate::{
    config::{KERNEL_INIT_STACK_SIZE, TRAP_CONTEXT},
    mm::{address::PhysAddr, memory_set::MemorySet},
    trap::{context::TrapContext, init_app_trap_return},
    KERNEL_STACK,
};

use super::processor::switch_trampoline;

pub struct Context {
    pub memory_set: MemorySet,
    pub task_cx: TaskContext,
    pub trap_cx: PhysAddr,
    
}

impl Context {
    pub fn new(memory_set: MemorySet, trap_cx: TrapContext) -> Self {
        let cx_pa = memory_set.va_translate((TRAP_CONTEXT).into()).unwrap();
        let satp = memory_set.token();
        unsafe {
            *cx_pa.as_type() = trap_cx;
        }
        Self {
            memory_set,
            task_cx: TaskContext::goto_trap_return(trap_cx.ksp, satp),
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
    pub fn goto_trap_return(ksp: usize, satp: usize) -> Self {
        let mut result = Self {
            ra: init_app_trap_return as usize,
            ksp,
            s: [0; 12],
        };
        result.s[0] = satp;
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

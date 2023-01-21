use core::ops::{Index, IndexMut};

use alloc::boxed::Box;
use bitflags::bitflags;

use super::{processor::Schedule, tcb::TaskControlBlock, tigger::SignalWaiter};

pub const MAX_SIG: usize = 31;

bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    pub struct SignalFlags: u32 {
        const SIGDEF    = 1 << 0; // Default signal handling
        const SIGCONT   = 1 << 18;
        const SIGSTOP   = 1 << 19;
    }
}

pub fn is_handle_by_kernel(flag: SignalFlags) -> bool {
    [SignalFlags::SIGSTOP, SignalFlags::SIGCONT].contains(&flag)
}

#[derive(Debug, Default)]
pub struct SignalActions {
    pub table: [usize; MAX_SIG + 1],
}

impl Index<usize> for SignalActions {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}

impl IndexMut<usize> for SignalActions {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.table[index]
    }
}

impl SignalActions {
    pub fn get(&self, index: usize) -> Option<&usize> {
        self.table.get(index)
    }
}

#[derive(Default)]
pub struct Signal {
    /// 全局信号标志位，不允许嵌套信号处理
    pub global_mask: bool,
    pub mask: SignalFlags,
    pub actions: SignalActions,
}

pub trait SignalHandle {
    fn handle_signals(&self);
}

impl<T> SignalHandle for T
where
    T: Schedule,
{
    fn handle_signals(&self) {
        let task = self.current_task();
        let local = task.process.inner.read();
        let signals = *task.shared.signals.lock();
        for signal in [0, 18, 19] {
            let flag = SignalFlags::from_bits_truncate(1 << signal as u32);
            if (signals & local.signal.mask).contains(flag) {
                match flag {
                    SignalFlags::SIGSTOP => {
                        self.blocking_current(SignalWaiter::new(&task, SignalFlags::SIGCONT));
                    }
                    // `SIGCONT` 信号由 `tigger` 处理
                    SignalFlags::SIGCONT => (),
                    _ => {
                        task.set_user_signal_sret(signal);
                        return;
                    }
                }
            }
        }
    }
}

impl TaskControlBlock {
    pub fn set_user_signal_sret(&self, signal: usize) {
        let mut process = self.process.inner.write();
        let handler = process.signal.actions[signal];
        if handler != 0 {
            // 关闭信号接收
            process.signal.global_mask = false;
            // 关闭信号标志位
            *self.shared.signals.lock() ^= SignalFlags::from_bits_truncate(1 << signal);

            // 备份 trap context ，设置用户信号处理函数入口
            let trap_cx = unsafe { self.trap_context() };
            self.local.borrow_mut().trap_cx_backup = Some(Box::new(*trap_cx));
            trap_cx.sepc = handler;
            trap_cx.set_return(signal);
        }
    }
}

use crate::{
    task::{processor::{Hart, Schedule}, tigger::Timer},
    timer::get_time_ms,
};

pub(super) trait SysSync {
    fn sys_sleep(&self, ms: usize) -> isize;
}

impl<T: Hart> SysSync for T {
    fn sys_sleep(&self, ms: usize) -> isize {
        let time = get_time_ms();
        self.blocking_current(Timer::new(ms));
        (get_time_ms() - time) as isize
    }
}

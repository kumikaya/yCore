use riscv::register::time;

use crate::{config::{CLOCK_FREQ, TICK_FREQ}, sbi::set_timer};

#[inline]
pub fn get_time() -> usize {
    time::read()
}

#[inline]
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / 1000)
}

#[inline]
pub fn set_next_trigger() {
    set_timer(time::read() + CLOCK_FREQ / TICK_FREQ);
}
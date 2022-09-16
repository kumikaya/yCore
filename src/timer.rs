use riscv::register::time;

use crate::{config::CLOCK_FREQ, sbi::set_timer};

const TICK_FREQ: usize = 100;

pub fn get_time() -> usize {
    time::read()
}

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / 1000)
}

pub fn set_next_trigger() {
    set_timer(time::read() + CLOCK_FREQ / TICK_FREQ);
}
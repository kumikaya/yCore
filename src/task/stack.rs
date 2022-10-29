use crate::{
    config::{KERNEL_STACK_SIZE, APP_STACK_SIZE},
    stdlib::cell::STCell,
};

const USER_BASE_STACK_ADDRESS: usize = 0x84000000;
const STACK_SUM: usize = APP_STACK_SIZE + KERNEL_STACK_SIZE;
use lazy_static::*;

use super::allocater;

lazy_static! {
    static ref COUNT: STCell<usize> = STCell::new(0);
}

/// return (user stack, kernel stack)
pub fn allocate_stack() -> (usize, usize) {
    let mut count = COUNT.borrow_mut();
    let user_stack = USER_BASE_STACK_ADDRESS + (*count) * STACK_SUM;
    let kernel_stack = user_stack + APP_STACK_SIZE;
    *count += 1;
    let succ = allocater::malloc_at(user_stack, STACK_SUM);
    if succ {
        (user_stack + STACK_SUM, kernel_stack)
    } else {
        unimplemented!()
    }
}

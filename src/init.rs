use crate::{config::PAGE_SIZE, println};


pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let sbss = sbss as usize;
    let ebss = ebss as usize;
    unsafe {
        core::slice::from_raw_parts_mut(sbss as *mut u8, ebss - sbss).fill(0);
    }
}

extern "C" {
    fn stack_top();
    fn stack_bottom();
}

pub fn init_stack_guard() {
    let stack_top = stack_top as usize;
    unsafe {
        core::slice::from_raw_parts_mut(stack_top as *mut u8, PAGE_SIZE).fill(u8::MAX);
    }
}

pub fn stack_cover_test() {
    unsafe {
        let un_cover = core::slice::from_raw_parts(stack_top as *const u8, PAGE_SIZE).iter().any(|&x| x == u8::MAX);
        assert!(un_cover);
        println!("[passed] stack_cover_test");
    }
}
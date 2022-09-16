
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    ((sbss as usize)..(ebss as usize)).for_each(|p| {
        unsafe {
            (p as *mut u8).write_volatile(0);
        }
    })
}
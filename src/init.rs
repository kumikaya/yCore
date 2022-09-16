use log::info;


pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    info!("clear bss at [{:x}-{:x}]", sbss as usize, ebss as usize);
    ((sbss as usize)..(ebss as usize)).for_each(|p| {
        unsafe {
            (p as *mut u8).write_volatile(0);
        }
    })
}
use core::slice;

use log::info;

const MAX_APP_NUM: usize = 8;

unsafe fn copy_mem_uncheck(src: usize, dst: usize, len: usize) {
    info!("copy {:x} to {:x} with len {:X}", src, dst, len);
    // asm!("fence.i");
    // slice::from_raw_parts_mut(ptr as *mut u8, APP_SIZE_LIMIT).fill(0);
    let src_start = src as *const u8;
    let app_raw = slice::from_raw_parts(src_start as *const u8, len);
    let app_dst: &mut [u8] = slice::from_raw_parts_mut(dst as *mut u8, len);
    app_dst.copy_from_slice(app_raw);
}

pub fn get_apps() -> (usize, [usize; MAX_APP_NUM + 1]) {
    unsafe {
        extern "C" {
            fn _num_app();
        }
        let nums = (_num_app as *const usize).read_volatile();
        let mut app_starts: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
        app_starts[..nums + 1].copy_from_slice(slice::from_raw_parts(
            (_num_app as *const usize).add(1),
            nums + 1,
        ));
        (nums, app_starts)
    }
}

pub fn copy_mem(src: usize, dst: usize, len: usize) {
    assert!(src + len <= dst || dst <= src - len);
    unsafe { copy_mem_uncheck(src, dst, len) };
}


use core::slice;

const MAX_APP_NUM: usize = 8;

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


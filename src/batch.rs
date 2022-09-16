const USER_STACK_SIZE: usize = 4 * 4096;
const KERNEL_STACK_SIZE: usize = 4 * 4096;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x200000;

struct AppManager {
    nums: usize,
    current: usize,
    app_starts: [usize; MAX_APP_NUM + 1],
}

static USER_STACK: [u8; USER_STACK_SIZE] = [0; USER_STACK_SIZE];
static KERNEL_STACK: [u8; KERNEL_STACK_SIZE] = [0; KERNEL_STACK_SIZE];

fn push_context(sp: usize, cx: TrapContext) -> usize {
    let sp = sp - size_of::<TrapContext>();
    let cx_ptr = sp as *mut TrapContext;
    unsafe {
        *cx_ptr = cx;
        cx_ptr as usize
    }
    
}

use core::{arch::asm, mem::size_of, slice};

use lazy_static::*;

use crate::{println, stdlib::cell::STCell, trap::TrapContext};
lazy_static! {
    static ref APP_MANAGER: STCell<AppManager> = {
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
            let result = AppManager {
                nums,
                current: 0,
                app_starts,
            };
            STCell::new(result)
        }
    };
}

impl AppManager {
    unsafe fn load_app(&self, id: usize) {
        if self.nums <= id {
            println!("All app completed!");
            panic!("Shutdown");
        }
        println!("[Kernel] load app_{}", id);
        asm!("fence.i");
        slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_start = self.app_starts[id];
        let app_end = self.app_starts[id + 1];
        let app_raw = slice::from_raw_parts(app_start as *const u8, app_end - app_start);
        let app_dst: &mut [u8] =
            slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_raw.len());
        app_dst.copy_from_slice(app_raw);
    }
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.borrow_mut();
    unsafe {
        app_manager.load_app(app_manager.current);
    }
    app_manager.current += 1;
    drop(app_manager);

    extern "C" {
        fn __restore(cx_addr: usize);
    }

    unsafe {
        __restore(push_context(
            &KERNEL_STACK as *const u8 as usize,
            TrapContext::init(APP_BASE_ADDRESS, &USER_STACK as *const u8 as usize),
        ));
    }

    panic!("Unreachable")
}

pub fn init() {
    let ksp = &KERNEL_STACK as *const u8 as usize;
    println!("Kernel stack: [{:x}-{:x}]", ksp, ksp + KERNEL_STACK_SIZE);
    let usp = &USER_STACK as *const u8 as usize;
    println!("User stack:   [{:x}-{:x}]", usp, usp + USER_STACK_SIZE);
}
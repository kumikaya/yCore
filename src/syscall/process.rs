use crate::{println, batch::run_next_app};


pub fn sys_exit(code: i32) -> !{
    println!("[Kernel] App exit with code {}", code);
    run_next_app()
}
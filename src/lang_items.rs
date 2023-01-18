use crate::{println, sbi};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    sbi::shutdown()
}

#[cfg(target_pointer_width = "64")]
pub mod half {
    type Halfusize = u32;
    type Halfisize = i32;
    type Qtrusize = u16;
    type Qtrisize = i16;
}

#[cfg(target_pointer_width = "32")]
pub mod half {
    type Halfusize = u16;
    type Halfisize = i16;
    type Qtrusize = u8;
    type Qtrisize = i8;
}

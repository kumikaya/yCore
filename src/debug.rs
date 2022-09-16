use crate::{println, print, sgr};

#[allow(unused)]
pub fn segment_info() {
    extern "C" {
        fn stext();
        fn etext();
        fn sdata();
        fn edata();
        fn srodata();
        fn erodata();
        fn sbss();
        fn ebss();
    }
    print!("{}", sgr!(GreenB));
    println!("Segment info:");
    print!("{}", sgr!(Green));
    println!("  text:   {:x}-{:x}", stext as usize, etext as usize);
    println!("  data:   {:x}-{:x}", sdata as usize, edata as usize);
    println!("  rodata: {:x}-{:x}", srodata as usize, erodata as usize);
    println!("  bss:    {:x}-{:x}", sbss as usize, ebss as usize);
    print!("{}", sgr!(DefaultColor));
}
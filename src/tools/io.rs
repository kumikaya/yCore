use crate::sbi;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            sbi::console_putchar(c as usize);
        }
        Ok(())
    }
}
#[allow(unused)]
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}



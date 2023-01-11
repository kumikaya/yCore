use super::File;
use crate::{mem::page_table::UserBuffer, print, sbi::console_getchar};

pub struct Stdin;

impl File for Stdin {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
    fn read(&self, buffer: &mut UserBuffer) -> usize {
        let ch: u8 = loop {
            let c = console_getchar();
            if c == 0 {
                // _yield();
            } else {
                break c as u8;
            }
        };
        buffer.write(&[ch]);
        1
    }
    fn write(&self, _buffer: &UserBuffer) -> usize {
        panic!("Can not write to stdin!");
    }
}

pub struct Stdout;

impl File for Stdout {
    fn readable(&self) -> bool {
        false
    }

    fn writable(&self) -> bool {
        true
    }

    fn read(&self, _buffer: &mut UserBuffer) -> usize {
        panic!("Cannot read from stdout!");
    }

    fn write(&self, buffer: &UserBuffer) -> usize {
        for buffer in buffer.buffers.iter() {
            print!("{}", core::str::from_utf8(*buffer).unwrap());
        }
        buffer.len()
    }
}

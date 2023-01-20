use super::File;
use crate::{mm::page_table::BufferHandle, print, sbi::console_getchar, task::processor::yield_};

pub struct Stdin;

impl File for Stdin {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
    fn read(&self, mut buffer_handle: BufferHandle) -> usize {
        let ch: u8 = loop {
            let c = console_getchar();
            if c == 0 {
                yield_();
            } else {
                break c as u8;
            }
        };
        buffer_handle.write(&[ch]);
        1
    }
    fn write(&self, _buffer_handle: BufferHandle) -> usize {
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

    fn read(&self, _buffer_handle: BufferHandle) -> usize {
        panic!("Cannot read from stdout!");
    }

    fn write(&self, buffer_handle: BufferHandle) -> usize {
        for buffer in buffer_handle.buffers.iter() {
            print!("{}", core::str::from_utf8(buffer).unwrap());
        }
        buffer_handle.len()
    }
}

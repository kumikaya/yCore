use alloc::sync::{Arc, Weak};
use anyhow::Result;
use spin::Mutex;

use crate::{mm::page_table::BufferHandle, task::processor::yield_};

use super::File;

pub struct Pipe {
    readable: bool,
    writable: bool,
    buffer: Arc<Mutex<PipeBuffer>>,
}

impl Pipe {
    pub fn read_end_with_buffer(buffer: Arc<Mutex<PipeBuffer>>) -> Self {
        Self {
            readable: true,
            writable: false,
            buffer,
        }
    }
    pub fn write_end_with_buffer(buffer: Arc<Mutex<PipeBuffer>>) -> Self {
        Self {
            readable: false,
            writable: true,
            buffer,
        }
    }
}

const RING_BUFFER_SIZE: usize = 128 + 1;

pub struct PipeBuffer {
    arr: [u8; RING_BUFFER_SIZE],
    head: usize,
    tail: usize,
    write_end: Option<Weak<Pipe>>,
}

impl PipeBuffer {
    pub fn new() -> Self {
        Self {
            arr: [0; RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            write_end: None,
        }
    }
    pub fn set_write_end(&mut self, write_end: &Arc<Pipe>) {
        self.write_end = Some(Arc::downgrade(write_end));
    }
    pub fn all_write_ends_closed(&self) -> bool {
        self.write_end.as_ref().unwrap().upgrade().is_none()
    }
    pub fn write(&mut self, byte: u8) -> Result<()> {
        if !self.is_full() {
            self.arr[self.tail] = byte;
            self.tail = (self.tail + 1) % RING_BUFFER_SIZE;
            Ok(())
        } else {
            Err(anyhow!("The buffer is full"))
        }
    }
    pub fn read(&mut self) -> Option<u8> {
        if !self.is_empty() {
            let c = self.arr[self.head];
            self.head = (self.head + 1) % RING_BUFFER_SIZE;
            Some(c)
        } else {
            None
        }
    }
    pub fn is_empty(&self) -> bool {
        self.tail == self.head
    }
    pub fn is_full(&self) -> bool {
        (self.tail + 1) % RING_BUFFER_SIZE == self.head
    }
}

impl File for Pipe {
    fn readable(&self) -> bool {
        self.readable
    }

    fn writable(&self) -> bool {
        self.writable
    }

    fn read(&self, buffer_handle: BufferHandle) -> usize {
        assert!(self.readable());
        println!("read: {}", buffer_handle.len());
        let mut read_len = 0;
        let mut pipe_buffer = self.buffer.lock();
        for x in buffer_handle.into_iter() {
            loop {
                if let Some(byte) = pipe_buffer.read() {
                    read_len += 1;
                    unsafe { *x = byte };
                    break;
                } else if pipe_buffer.all_write_ends_closed() {
                    return read_len;
                } else {
                    drop(pipe_buffer);
                    yield_();
                    pipe_buffer = self.buffer.lock();
                }
            }
        }
        read_len
    }

    fn write(&self, buffer_handle: BufferHandle) -> usize {
        assert!(self.writable());
        println!("write: {}", buffer_handle.len());
        let mut writed_len = 0;
        let mut pipe_buffer = self.buffer.lock();
        for x in buffer_handle.into_iter() {
            loop {
                if let Ok(_) = pipe_buffer.write(unsafe { *x }) {
                    writed_len += 1;
                    break;
                } else {
                    drop(pipe_buffer);
                    yield_();
                    pipe_buffer = self.buffer.lock();
                }
            }
        }
        writed_len
    }
}

pub fn make_pipe() -> (Arc<Pipe>, Arc<Pipe>) {
    let buffer = Arc::new(Mutex::new(PipeBuffer::new()));
    let read_end = Arc::new(Pipe::read_end_with_buffer(buffer.clone()));
    let write_end = Arc::new(Pipe::write_end_with_buffer(buffer.clone()));
    buffer.lock().set_write_end(&write_end);
    (read_end, write_end)
}

#[allow(unused)]
pub fn ring_buffer_test() {
    let mut buffer = PipeBuffer::new();
    for i in 0..(RING_BUFFER_SIZE as u8 - 1) {
        buffer.write(i).unwrap();
    }
    assert!(buffer.write(0).is_err());
    for i in 0..(RING_BUFFER_SIZE as u8 - 1) {
        assert_eq!(buffer.read().unwrap(), i);
    }
    assert!(buffer.read().is_none());
    println!("Ring buffer test pass!");
}

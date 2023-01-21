use core::{
    cell::UnsafeCell,
    hint,
    mem::MaybeUninit,
    sync::atomic::{AtomicUsize, Ordering},
};

use alloc::sync::Arc;

const RING_BUFFER_SIZE: usize = 32;

pub struct RingBuffer<T> {
    head: AtomicUsize,
    tail: AtomicUsize,
    buf: UnsafeCell<[MaybeUninit<T>; RING_BUFFER_SIZE]>,
}

unsafe impl<T: Send> Send for RingBuffer<T> {}

impl<T> RingBuffer<T> {
    pub const fn new() -> Self {
        Self {
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            buf: UnsafeCell::new(MaybeUninit::uninit_array::<RING_BUFFER_SIZE>()),
        }
    }
    fn capacity(&self) -> usize {
        unsafe { (*self.buf.get()).len() }
    }

    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        let cap = self.capacity();
        (cap + tail - head) % cap
    }

    pub fn is_empty(&self) -> bool {
        self.tail.load(Ordering::Relaxed) == self.head.load(Ordering::Relaxed)
    }
    pub fn is_full(&self) -> bool {
        let tail = (self.tail.load(Ordering::Relaxed) + 1) % self.capacity();
        tail == self.head.load(Ordering::Relaxed)
    }
    fn write(&self, data: T) -> Option<T> {
        let trail = self.tail.load(Ordering::Relaxed);
        let new_trail = (trail + 1) % self.capacity();
        let result = if new_trail != self.head.load(Ordering::Acquire) {
            unsafe { (*self.buf.get())[trail].write(data) };
            self.tail.store(new_trail, Ordering::Release);
            None
        } else {
            Some(data)
        };
        result
    }
    fn read(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);

        if head != self.tail.load(Ordering::Acquire) {
            let c = unsafe { (*self.buf.get())[head].assume_init_read() };
            self.head
                .store((head + 1) % self.capacity(), Ordering::Release);
            Some(c)
        } else {
            None
        }
    }
}

pub fn channel<T: 'static>() -> (Reciver<T>, Sender<T>) {
    let rb = Arc::new(RingBuffer::new());
    unsafe { (Reciver::new(rb.clone()), Sender::new(rb)) }
}

#[derive(Clone)]
pub struct Sender<T> {
    rb: Arc<RingBuffer<T>>,
}

pub struct Reciver<T> {
    rb: Arc<RingBuffer<T>>,
}

unsafe impl<T: Send> Send for Sender<T> {}
unsafe impl<T: Send> Send for Reciver<T> {}

fn spin_loops(count: usize) {
    for _ in 0..count {
        hint::spin_loop();
    }
}

impl<T> Sender<T> {
    unsafe fn new(rb: Arc<RingBuffer<T>>) -> Self {
        Self { rb }
    }
    pub fn send(&self, data: T) {
        let mut data = Some(data);
        loop {
            data = self.rb.write(data.take().unwrap());
            if data.is_none() {
                break;
            }
            hint::spin_loop();
        }
    }
    pub fn try_send(&self, data: T) -> Option<T> {
        self.rb.write(data)
    }

    pub fn available(&self) -> usize {
        self.rb.capacity() - self.rb.len() - 1
    }
}

impl<T> Reciver<T> {
    unsafe fn new(rb: Arc<RingBuffer<T>>) -> Self {
        Self { rb }
    }
    pub fn recv(&self) -> T {
        loop {
            if let Some(data) = self.rb.read() {
                return data;
            }
            hint::spin_loop();
        }
    }
    pub fn try_recv(&self) -> Option<T> {
        self.rb.read()
    }

    pub fn available(&self) -> usize {
        self.rb.len()
    }
}

// pub fn channel_test() {
//     let (rx, tx) = channel::<u8>();
//     assert_eq!(rx.available(), 0);
//     assert_eq!(tx.available(), RING_BUFFER_SIZE - 1);
//     tx.send(0);
//     assert_eq!(rx.available(), 1);
//     assert_eq!(tx.available(), RING_BUFFER_SIZE - 2);
// }

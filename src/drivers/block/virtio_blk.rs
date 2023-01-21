use core::ptr::NonNull;

use alloc::vec::Vec;
use easy_fs::BlockDevice;
use spin::{Lazy, Mutex};
use virtio_drivers::{
    device::blk::VirtIOBlk,
    transport::mmio::{MmioTransport, VirtIOHeader},
    BufferDirection, Hal,
};

use crate::mm::{
    address::{PhysAddr, PhysPageNum},
    frame_allocator::{frame_alloc, frame_dealloc, FrameTracker},
    memory_set::KERNEL_SPACE,
};

#[allow(unused)]
const VIRTIO0: usize = 0x10001000;

pub struct VirtIOBlock {
    inner: Mutex<VirtIOBlk<VirtioHal, MmioTransport>>,
}

unsafe impl Sync for VirtIOBlock {}
unsafe impl Send for VirtIOBlock {}

static QUEUE_FRAMES: Lazy<Mutex<Vec<FrameTracker>>> = Lazy::new(|| Mutex::new(Vec::new()));

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.inner
            .lock()
            .read_blocks(block_id, buf)
            .expect("Error when reading VirtIOBlk");
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.inner
            .lock()
            .write_blocks(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}

impl VirtIOBlock {
    #[allow(unused)]
    pub fn new() -> Self {
        let header = NonNull::new(VIRTIO0 as *mut VirtIOHeader).unwrap();
        unsafe {
            let transport = MmioTransport::new(header).unwrap();
            Self {
                inner: Mutex::new(VirtIOBlk::new(transport).unwrap()),
            }
        }
    }
}

pub struct VirtioHal;

unsafe impl Hal for VirtioHal {
    fn dma_alloc(pages: usize, _direction: BufferDirection) -> (usize, NonNull<u8>) {
        let mut ppn_base = PhysPageNum::default();
        for i in 0..pages {
            let frame = frame_alloc().unwrap();
            if i == 0 {
                ppn_base = frame.ppn;
            }
            assert_eq!(usize::from(frame.ppn), usize::from(ppn_base) + i);
            QUEUE_FRAMES.lock().push(frame);
        }
        let paddr = ppn_base.start().into();
        let vaddr = NonNull::new(paddr as _).unwrap();
        (paddr, vaddr)
    }

    unsafe fn dma_dealloc(paddr: usize, _vaddr: NonNull<u8>, pages: usize) -> i32 {
        let ppn_base: PhysPageNum = PhysAddr::from(paddr).into();
        for ppn in ppn_base..ppn_base.offset(pages as isize) {
            frame_dealloc(ppn);
        }
        0
    }

    unsafe fn mmio_phys_to_virt(paddr: usize, _size: usize) -> NonNull<u8> {
        NonNull::new(paddr as _).unwrap()
    }


    unsafe fn share(buffer: NonNull<[u8]>, _direction: BufferDirection) -> usize {
        let vaddr = buffer.as_ptr() as *mut u8 as usize;
        KERNEL_SPACE
            .lock()
            .va_translate(vaddr.into())
            .unwrap()
            .into()
    }

    unsafe fn unshare(_paddr: usize, _buffer: NonNull<[u8]>, _direction: BufferDirection) {}
}

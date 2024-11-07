mod virtio_blk;
use crate::{board::BlockDeviceImpl, println};
use alloc::sync::Arc;
use easy_fs::BlockDevice;
use spin::Lazy;
pub use virtio_blk::VirtIOBlock;

pub static BLOCK_DEVICE: Lazy<Arc<dyn BlockDevice>> =
    Lazy::new(|| Arc::new(BlockDeviceImpl::new()));

#[allow(unused)]
pub fn block_device_test() {
    let block_device = BLOCK_DEVICE.clone();
    let mut write_buffer = [0u8; 512];
    let mut read_buffer = [0u8; 512];
    for i in 0..512 {
        for byte in write_buffer.iter_mut() {
            *byte = i as u8;
        }
        block_device.write_block(i as usize, &write_buffer);
        block_device.read_block(i as usize, &mut read_buffer);
        assert_eq!(write_buffer, read_buffer);
    }
    println!("block device test passed!");
}

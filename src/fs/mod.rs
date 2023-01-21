use crate::mm::page_table::BufferHandle;
use alloc::sync::Arc;
use bitflags::bitflags;

pub mod inode;
pub mod pipe;
pub mod stdio;
pub type FileBox = Arc<dyn File + Send + Sync>;

pub trait File {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    /// 读取文件到 `BufferHandle` ，返回读取长度
    fn read(&self, buffer_handle: BufferHandle) -> usize;
    /// 写入 `BufferHandle` 到文件，返回写入长度
    fn write(&self, buffer_handle: BufferHandle) -> usize;
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FileFlags: u8 {
        const R = 1 << 0;
        const W = 1 << 1;
        const RW  = Self::R.bits() | Self::W.bits();
    }
}

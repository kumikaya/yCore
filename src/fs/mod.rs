use alloc::sync::Arc;
use crate::mem::page_table::UserBuffer;

pub mod stdio;
pub type FileBox = Arc<dyn File + Send + Sync>;

pub trait File {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    /// 读取文件到 `UserBuffer` ，返回读取长度
    fn read(&self, buffer: &mut UserBuffer) -> usize;
    /// 写入 `UserBuffer` 到文件，返回写入长度
    fn write(&self, buffer: &UserBuffer) -> usize;
}
use crate::{drivers::block::BLOCK_DEVICE, mm::page_table::BufferHandle, println, task::task::TaskControlBlock};
use alloc::{sync::Arc, vec::Vec};
use easy_fs::{EasyFileSystem, Inode, FileType};
use lazy_static::lazy_static;
use bitflags::bitflags;
use spin::Mutex;
use xmas_elf::ElfFile;

use super::{File, FileFlags};

lazy_static! {
    pub static ref ROOT_INODE: Arc<Inode> = {
        let efs = EasyFileSystem::open(BLOCK_DEVICE.clone());
        Arc::new(EasyFileSystem::root_inode(&efs))
    };
}

/// List all files in the filesystems
pub fn list_apps() {
    println!("-APPS----");
    for app in ROOT_INODE.ls() {
        println!("{}", app);
    }
    println!("---------");
}

pub struct OSInode {
    perm: FileFlags,
    inner: Mutex<OSInodeInner>,
}

pub struct OSInodeInner {
    offset: usize,
    inode: Arc<Inode>,
}

impl OSInode {
    pub fn new(perm: FileFlags, inode: Arc<Inode>) -> Self {
        Self {
            perm,
            inner: Mutex::new(OSInodeInner::new(inode)),
        }
    }
    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.lock();
        let mut buffer = [0u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            if len == 0 {
                break;
            }
            inner.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }
}

impl OSInodeInner {
    pub fn new(inode: Arc<Inode>) -> Self {
        Self { offset: 0, inode }
    }
}

impl File for OSInode {
    fn readable(&self) -> bool {
        self.perm.contains(FileFlags::R)
    }

    fn writable(&self) -> bool {
        self.perm.contains(FileFlags::W)
    }

    fn read(&self, mut buffer_handle: BufferHandle) -> usize {
        let mut inner = self.inner.lock();
        let base_offset = inner.offset;
        for buffer in buffer_handle.buffers.iter_mut() {
            let read_size = inner.inode.read_at(inner.offset, *buffer);
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
        }
        inner.offset - base_offset
    }

    fn write(&self, buffer_handle: BufferHandle) -> usize {
        let mut inner = self.inner.lock();
        let base_offset = inner.offset;
        for buffer in &buffer_handle.buffers {
            let wrtie_size = inner.inode.write_at(inner.offset, *buffer);
            assert_eq!(wrtie_size, buffer.len());
            inner.offset += wrtie_size;
        }
        inner.offset - base_offset
    }
}

bitflags! {
    ///Open file flags
    pub struct OpenFlags: u8 {
        ///Read only
        const RDONLY = 1 << 0;
        ///Write only
        const WRONLY = 1 << 1;
        ///Read & Write
        const RDWR = Self::RDONLY.bits | Self::WRONLY.bits;
        ///Allow create
        const CREATE = 1 << 2;
        ///Clear file and return an empty one
        const TRUNC = 1 << 3;
    }
}


pub fn inode_test() {
    let test_dir = ROOT_INODE.create("test", FileType::Directory).unwrap();
    let file = test_dir.create("hello.txt", FileType::File).unwrap();
    const STR: &str = "Hello World!";
    file.write_at(0, STR.as_bytes());
    let buffer = &mut [0; STR.len()];
    ROOT_INODE.find("test").unwrap().find("hello.txt").unwrap().read_at(0, buffer);
    assert_eq!(core::str::from_utf8(buffer).unwrap(), STR)
}

impl OpenFlags {
    pub fn get_perm(&self) -> FileFlags {
        FileFlags::from_bits_truncate(self.bits)
    }
}

pub fn open_file(path: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
    let perm = flags.get_perm();
    if flags.contains(OpenFlags::CREATE) {
        if let Some(inode) = ROOT_INODE.find(path) {
            inode.clear();
            Some(Arc::new(OSInode::new(perm, inode)))
        } else {
            ROOT_INODE.create(path, FileType::File).map(|inode| {
                Arc::new(OSInode::new(perm, inode))
            })
        }
    } else {
        ROOT_INODE.find(path).map(|inode| {
            if flags.contains(OpenFlags::TRUNC) {
                inode.clear()
            }
            Arc::new(OSInode::new(perm, inode))
        })
    }
}

pub fn open_app(path: &str, parent: Option<&Arc<TaskControlBlock>>) -> Option<Arc<TaskControlBlock>> {
    if let Some(app_inode) = open_file(&path, OpenFlags::RDONLY) {
        let app_data = app_inode.read_all();
        let elf = ElfFile::new(app_data.as_slice()).unwrap();
        let task = TaskControlBlock::from_elf(elf);
        if let Some(parent) = parent {
            task.set_parent(parent);
        }
        Some(task)
    } else {
        None
    }
}
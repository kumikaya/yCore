use core::ops::{Sub, SubAssign, Add};
use core::{ops::AddAssign, mem::size_of, slice};
use core::fmt::{Debug, Display};
use crate::config::{PAGE_SIZE, PAGE_WIDTH, SV39_PAGE_LEVEL, SV39_PAGE_INDEX_WIDTH};

use super::page_table::PageTableEntry;

const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_WIDTH;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

macro_rules! impl_add_and_sub {
    ($type: ty) => {
        impl const Add for $type {
            type Output = Self;
        
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        impl AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }
        impl const Sub for $type {
            type Output = Self;
        
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        impl SubAssign for $type {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }
    };
}

macro_rules! impl_offset {
    ($type: ty) => {
        impl $type {
            pub const fn offset(self, val: isize) -> Self {
                Self(self.0 + (val as usize))
            }
        }
    };
}

macro_rules! impl_display {
    ($type: ty) => {
        impl Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }        
    };
}

impl_add_and_sub!(PhysAddr);
impl_add_and_sub!(VirtAddr);
impl_add_and_sub!(PhysPageNum);
impl_add_and_sub!(VirtPageNum);

impl_offset!(PhysAddr);
impl_offset!(VirtAddr);
impl_offset!(PhysPageNum);
impl_offset!(VirtPageNum);

impl_display!(PhysAddr);
impl_display!(VirtAddr);
impl_display!(PhysPageNum);
impl_display!(VirtPageNum);


impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}
impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {
        v.0
    }
}
impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0
    }
}


impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self {
        Self(v.0 << PAGE_WIDTH)
    }
}

impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}
impl From<usize> for VirtPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(v: VirtAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(v: VirtPageNum) -> Self {
        Self(v.0 << PAGE_WIDTH)
    }
}

impl From<VirtAddr> for usize {
    fn from(v: VirtAddr) -> Self {
        v.0
    }
}
impl From<VirtPageNum> for usize {
    fn from(v: VirtPageNum) -> Self {
        v.0
    }
}


impl PhysAddr {
    pub fn floor(self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
    pub fn page_offset(self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn is_page_align(self) -> bool {
        self.page_offset() == 0
    }

}

impl VirtPageNum {
    pub fn indexs(self) -> [usize; SV39_PAGE_LEVEL] {
        let mut result = [0usize; SV39_PAGE_LEVEL];
        let mut vpn = self.0;   
        for i in (0..SV39_PAGE_LEVEL).rev() {
            result[i] = vpn & ((1 << SV39_PAGE_INDEX_WIDTH) - 1);
            vpn >>= SV39_PAGE_INDEX_WIDTH;
        }
        result
    }

}

impl VirtAddr {
    pub fn floor(self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 + (PAGE_SIZE - 1)) / PAGE_SIZE)
    }
    pub fn page_offset(self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn is_page_align(self) -> bool {
        self.page_offset() == 0
    }
}

impl PhysPageNum {
    pub unsafe fn as_pte_array(self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = self.into();
        unsafe {
            slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, PAGE_SIZE / size_of::<PageTableEntry>())
        }
    }

    pub unsafe fn as_bytes(self) -> &'static mut [u8] {
        let pa: PhysAddr = self.into();
        unsafe {
            slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE)
        }
    }

}


pub trait StepByOne {
    fn step(&mut self);
}
impl StepByOne for VirtPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}
impl StepByOne for PhysPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

#[derive(Copy, Clone)]
/// a simple range structure for type T
pub struct SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    l: T,
    r: T,
}
impl<T> SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start {:?} > end {:?}!", start, end);
        Self { l: start, r: end }
    }
    pub fn get_start(&self) -> T {
        self.l
    }
    pub fn get_end(&self) -> T {
        self.r
    }
}
impl<T> IntoIterator for SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    type IntoIter = SimpleRangeIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator::new(self.l, self.r)
    }
}
/// iterator for the simple range structure
pub struct SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    current: T,
    end: T,
}
impl<T> SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(l: T, r: T) -> Self {
        Self { current: l, end: r }
    }
}
impl<T> Iterator for SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}

/// a simple range structure for virtual page number
pub type VPNRange = SimpleRange<VirtPageNum>;
pub type PPNRange = SimpleRange<PhysPageNum>;

impl<T> Debug for SimpleRange<T>
where
    T: StepByOne + Debug + Copy + PartialOrd,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SimpleRange").field("l", &self.l).field("r", &self.r).finish()
    }
}
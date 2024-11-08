use crate::config::{PAGE_SIZE, PAGE_WIDTH, SV39_PAGE_INDEX_WIDTH, SV39_PAGE_LEVEL};
use core::fmt::{Debug, Display};
use core::iter::Step;
use core::ops::{Add, Range, Sub, SubAssign};
use core::{mem::size_of, ops::AddAssign, slice};

use super::page_table::PageTableEntry;

const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_WIDTH;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_WIDTH;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(usize);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(usize);

macro_rules! impl_add_sub {
    ($($type: ty),*) => {
        $(
            impl Add for $type {
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
            impl Sub for $type {
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
        )*
    };
}

macro_rules! impl_offset {
    ($($type: ty),*) => {
        $(
            impl $type {
                pub const fn offset(self, val: isize) -> Self {
                    Self(self.0 + (val as usize))
                }
            }
        )*
    };
}

macro_rules! impl_display {
    ($($type: ty),*) => {
        $(
            impl Display for $type {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{:#x}", self.0)
                }
            }
        )*
    };
}

macro_rules! impl_step {
    ($($type: ty),*) => {
        $(
            impl Step for $type {
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if end >= start {
                        Some(end.0 - start.0)
                    } else {
                        None
                    }
                }

                fn forward_checked(start: Self, count: usize) -> Option<Self> {
                    Some(start + count.into())
                }

                fn backward_checked(start: Self, count: usize) -> Option<Self> {
                    Some(start - count.into())
                }
        }
        )*
    };
}

impl_add_sub!(PhysAddr, VirtAddr, PhysPageNum, VirtPageNum);

impl_offset!(PhysAddr, VirtAddr, PhysPageNum, VirtPageNum);

impl_display!(PhysAddr, VirtAddr, PhysPageNum, VirtPageNum);

impl_step!(PhysAddr, VirtAddr, VirtPageNum, PhysPageNum);

pub type VPNRange = Range<VirtPageNum>;

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        assert!(v < (1 << PA_WIDTH_SV39));
        Self(v)
    }
}
impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        assert!(v < (1 << PPN_WIDTH_SV39));
        Self(v)
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
        assert!(isize::abs(v as isize) < (1 << VA_WIDTH_SV39));
        Self(v)
    }
}
impl From<usize> for VirtPageNum {
    fn from(v: usize) -> Self {
        assert!(isize::abs(v as isize) < (1 << VPN_WIDTH_SV39));
        Self(v)
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
    pub unsafe fn as_type<T>(&self) -> &'static mut T {
        // 类型T不能跨页
        assert!(
            size_of::<T>() < PAGE_SIZE
                && self.page_offset() < PhysAddr(self.0 + size_of::<T>()).page_offset()
        );
        (self.0 as *mut T).as_mut().unwrap()
    }

    pub fn floor(self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }
    pub fn page_offset(self) -> usize {
        self.0 % PAGE_SIZE
    }
}

impl VirtPageNum {
    pub fn start(self) -> VirtAddr {
        self.into()
    }
    pub fn range(self) -> Range<VirtAddr> {
        let start = self.into();
        let end = (self + VirtPageNum(1)).into();
        start..end
    }
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
        VirtPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }
    pub fn page_offset(self) -> usize {
        self.0 % PAGE_SIZE
    }
}

impl PhysPageNum {
    pub fn start(self) -> PhysAddr {
        self.into()
    }
    pub fn range(self) -> Range<PhysAddr> {
        let start = self.into();
        let end = (self + PhysPageNum(1)).into();
        start..end
    }
    pub unsafe fn as_pte_array(self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = self.into();
        slice::from_raw_parts_mut(
            pa.0 as *mut PageTableEntry,
            PAGE_SIZE / size_of::<PageTableEntry>(),
        )
    }

    pub unsafe fn as_bytes(self) -> &'static mut [u8] {
        let pa: PhysAddr = self.into();
        slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE)
    }
}

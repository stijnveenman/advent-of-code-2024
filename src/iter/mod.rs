use debug_iterator::{debug_iterator, DebugIter};
use u32_iterator::{u32_iterator, U32Iter};

pub mod debug_iterator;
mod u32_iterator;

pub trait AocItertools: Iterator {
    fn dbg(self) -> DebugIter<Self>
    where
        Self: Sized,
    {
        debug_iterator(self)
    }

    fn u32(self) -> U32Iter<Self>
    where
        Self: Sized,
    {
        u32_iterator(self)
    }
}

impl<T> AocItertools for T where T: Iterator + ?Sized {}

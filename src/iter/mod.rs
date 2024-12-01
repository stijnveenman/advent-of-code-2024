use debug_iterator::{debug_iterator, DebugIter};

pub mod debug_iterator;

pub trait AocItertools: Iterator {
    fn dbg(self) -> DebugIter<Self>
    where
        Self: Sized,
    {
        debug_iterator(self)
    }
}

impl<T> AocItertools for T where T: Iterator + ?Sized {}

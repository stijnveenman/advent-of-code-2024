use std::fmt::Debug;

pub struct DebugIter<T> {
    i: T,
}

impl<T> Iterator for DebugIter<T>
where
    T: Iterator,
    T::Item: Debug,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.i.next()?;
        println!("{:?}", r);
        Some(r)
    }
}

pub trait DbgIterator: Iterator {
    fn dbg(self) -> DebugIter<Self>
    where
        Self: Sized,
    {
        DebugIter { i: self }
    }
}

impl<T> DbgIterator for T where T: Iterator + ?Sized {}

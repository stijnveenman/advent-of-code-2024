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

pub fn debug_iterator<T>(i: T) -> DebugIter<T>
where
    T: IntoIterator,
{
    DebugIter { i }
}

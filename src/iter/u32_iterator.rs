pub struct U32Iter<T> {
    i: T,
}

impl<'a, T> Iterator for U32Iter<T>
where
    T: Iterator<Item = &'a str>,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.i.next()?;

        match r.trim().parse::<u32>() {
            Ok(v) => Some(v),
            Err(e) => panic!("failed to parse u32 for [{}]: {}", r, e),
        }
    }
}

pub fn u32_iterator<T>(i: T) -> U32Iter<T>
where
    T: IntoIterator,
{
    U32Iter { i }
}

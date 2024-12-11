pub struct UsizeIter<T> {
    i: T,
}

impl<'a, T> Iterator for UsizeIter<T>
where
    T: Iterator<Item = &'a str>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.i.next()?;

        match r.parse::<usize>() {
            Ok(v) => Some(v),
            Err(e) => panic!("failed to parse usize for [{}]: {}", r, e),
        }
    }
}

pub fn usize_iterator<T>(i: T) -> UsizeIter<T>
where
    T: IntoIterator,
{
    UsizeIter { i }
}
